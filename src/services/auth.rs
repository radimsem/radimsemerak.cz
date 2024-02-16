use std::env;
use std::sync::Arc;

use anyhow::anyhow;
use async_io::Timer;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use chrono::{Local, Duration, NaiveDateTime};
use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::{AppDataResponse, AppState};
use crate::error::AppError;
use crate::models::token::Token;
use crate::schema::tokens;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    pw: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    id: i32,
    content: String,
    expires: i64
}

#[derive(Deserialize)]
pub struct TokenValidationRequest {
    id: i32,
    client: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Claims {
    sub: String,
    company: String,
    exp: i64
}

pub struct TokensEncodeHandler {
    client: String,
    server: String
}

struct TokensDecodeHandler {
    client: TokenData<Claims>,
    server: TokenData<Claims>
}

type TokenExpirationHandler<T> = tokio::task::JoinHandle<Result<T, AppError>>;

pub async fn handle_login_auth(State(AppState { db }): State<AppState>, Json(LoginRequest { username, pw }): Json<LoginRequest>) -> Result<AppDataResponse<LoginResponse>, AppError> {
    if 
        username != env::var("ADMIN_USERNAME")? ||
        pw       != env::var("ADMIN_PASSWORD")?
    {
        return Err(AppError(
            anyhow!("Invalid username or password!"),
            StatusCode::UNAUTHORIZED
        ))
    }

    let curr = Local::now().naive_local();
    let expires = curr + Duration::hours(env::var("TOKEN_EXPIRATION_HOURS")?.parse()?);

    let content = encode(
        &Header::new(Algorithm::default()),
        &Claims { sub: env::var("JWT_SUBJECT")?, company: username, exp: expires.timestamp() },
        &EncodingKey::from_secret(env::var("JWT_SECRET")?.as_ref())
    )?;
    let token = Token {
        content,
        created_at: curr,
        expires
    };
    let (id, content, expires) = diesel::insert_into(tokens::table)
        .values(&token)
        .returning((tokens::id, tokens::content, tokens::expires))
        .get_result::<(i32, String, NaiveDateTime)>(&mut db.lock().await.conn)?;
        
    Ok((
        StatusCode::CREATED,
        Json(LoginResponse {
            id,
            content,
            expires: expires.timestamp()
        })
    ))
}

pub async fn verify_token(State(AppState { db }): State<AppState>, Json(TokenValidationRequest { id, client }): Json<TokenValidationRequest>) -> Result<(StatusCode, ()), AppError> {
    let token: Option<String> = tokens::table
        .find(id)
        .select(tokens::content)
        .first::<String>(&mut db.lock().await.conn)
        .optional()?;

    match token {
        Some(server) => {
            match is_valid(TokensEncodeHandler { client, server })? {
                true => Ok((StatusCode::OK, ())),
                false => Err(AppError(
                    anyhow!("Token is invalid!"),
                    StatusCode::UNAUTHORIZED
                ))
            }
        },
        None => Err(AppError(
            anyhow!("There are is not any valid token with id {id}"),
            StatusCode::NOT_FOUND
        ))
    }
}

pub async fn handle_tokens_expiration(State(AppState { db }): State<AppState>) -> Result<(StatusCode, ()), AppError> {
    let tokens: Vec<(i32, NaiveDateTime)> = tokens::table
        .select((tokens::id, tokens::expires))
        .load(&mut db.lock().await.conn)?;
    
    let mut handles: Vec<TokenExpirationHandler<usize>> = Vec::with_capacity(tokens.capacity());
    if tokens.len() > 0 {
        for (id, expiration) in tokens {
            let curr = Local::now().naive_local();

            if curr < expiration {
                let left = expiration - curr;
                let timer = Timer::after(std::time::Duration::from_secs(left.num_seconds().try_into()?));
                let db = Arc::clone(&db);

                handles.push(tokio::task::spawn(async move {
                    timer.await;
                    diesel::delete(FilterDsl::filter(tokens::table, tokens::id.eq(id)))
                        .execute(&mut db.lock().await.conn)
                        .map_err(|e| AppError(anyhow!("Could not delete token {id}: {e}"), StatusCode::EXPECTATION_FAILED))
                }))
            }
        }
    }

    for handle in handles {
        if let Err(err) = handle.await? { return Err(err) }
    }
    
    Ok((StatusCode::OK, ()))
}

fn is_valid(encode_handler: TokensEncodeHandler) -> anyhow::Result<bool> {
    let decode_handler = TokensDecodeHandler {
        client: handle_decode(encode_handler.client)?,
        server: handle_decode(encode_handler.server)?
    };

    Ok(decode_handler.client.claims == decode_handler.server.claims)
}

fn handle_decode(token: String) -> anyhow::Result<TokenData<Claims>> {
    let result = decode::<Claims>(
        token.as_str(),
        &DecodingKey::from_secret(env::var("JWT_SECRET")?.as_ref()),
        &Validation::new(Algorithm::default())
    )?;
    Ok(result)
}