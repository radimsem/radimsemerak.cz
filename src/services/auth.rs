use std::env;

use anyhow::anyhow;
use async_io::Timer;
use axum::extract::State;
use axum::{Json, http::StatusCode};
use chrono::{Local, Duration, NaiveDateTime};
use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl, prelude::*};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::AppState;
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
    expires: i64
}

pub struct TokensEncodeHandler {
    client: String,
    server: String
}

struct TokensDecodeHandler {
    client: TokenData<Claims>,
    server: TokenData<Claims>
}

type AppDataResponse<T> = (StatusCode, Json<T>);

pub async fn login_auth_handler(State(AppState { data }): State<AppState>, Json(LoginRequest { username, pw }): Json<LoginRequest>) -> Result<AppDataResponse<LoginResponse>, AppError> {
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
    let time = expires.timestamp();

    let content = encode(
        &Header::new(Algorithm::default()),
        &Claims { sub: env::var("JWT_SUBJECT")?, company: username, expires: time.clone() },
        &EncodingKey::from_secret(env::var("JWT_SECRET")?.as_ref())
    )?;
    let token = Token {
        content,
        created_at: curr,
        expires
    };
    let (id, content) = diesel::insert_into(tokens::table)
        .values(&token)
        .returning((tokens::id, tokens::content))
        .get_result::<(i32, String)>(&mut data.lock().unwrap().conn)?;
        
    Ok((
        StatusCode::CREATED,
        Json(LoginResponse {
            id,
            content,
            expires: time
        })
    ))
}

pub async fn validate_token(State(AppState { data }): State<AppState>, Json(TokenValidationRequest { id, client }): Json<TokenValidationRequest>) -> Result<(StatusCode, ()), AppError> {
    let token: Option<String> = tokens::table
        .find(id)
        .select(tokens::content)
        .first::<String>(&mut data.lock().unwrap().conn)
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

pub async fn handle_tokens_expiration(State(AppState { data }): State<AppState>) -> Result<(StatusCode, ()), AppError> {
    let tokens: Vec<(i32, NaiveDateTime)> = tokens::table.select((tokens::id, tokens::expires))
        .load(&mut data.lock().unwrap().conn)?;

    if tokens.len() > 0 {
        for (id, expiration) in tokens {
            let curr = Local::now().naive_local();

            if curr < expiration {
                let left = expiration - curr;
                let timer = Timer::after(std::time::Duration::from_secs(left.num_seconds().try_into()?));
                let db = data.clone();

                tokio::task::spawn(async move {
                    timer.await;

                    let _ = diesel::delete(FilterDsl::filter(tokens::table, tokens::id.eq(id)))
                        .execute(&mut db.lock().unwrap().conn)
                        .unwrap();
                });
            }
        }
    }
    
    Ok((StatusCode::OK, ()))
}

fn is_valid(encode_handler: TokensEncodeHandler) -> anyhow::Result<bool> {
    let decode_handler = TokensDecodeHandler {
        client: handle_decode(encode_handler.client)?,
        server: handle_decode(encode_handler.server)?
    };

    match decode_handler.client.claims == decode_handler.server.claims {
        true => Ok(true),
        false => Ok(false)
    }
}

fn handle_decode(token: String) -> anyhow::Result<TokenData<Claims>> {
    let result = decode::<Claims>(
        token.as_str(),
        &DecodingKey::from_secret(env::var("JWT_SECRET")?.as_ref()),
        &Validation::new(Algorithm::default())
    )?;
    Ok(result)
}