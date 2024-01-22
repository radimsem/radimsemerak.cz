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
    token: Option<TokenResponse>,
    err: Option<String>
}

#[derive(Serialize)]
pub struct TokenResponse {
    id: i32,
    content: String,
    expires: i64
}

#[derive(Deserialize)]
pub struct TokenValidationRequest {
    id: i32,
    client: String
}

#[derive(Serialize)]
pub struct TokenValidationResponse {
    validated: bool,
    err: Option<String>
}

#[derive(Serialize, Deserialize, PartialEq)]
struct Claims {
    sub: String,
    company: String
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
        return Ok((
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse {
                token: None,
                err: Some("Invalid username or password!".to_string())
            })
        ))
    }

    let content = encode(
        &Header::new(Algorithm::default()),
        &Claims { sub: env::var("JWT_SUBJECT")?, company: username },
        &EncodingKey::from_secret(env::var("JWT_SECRET")?.as_ref())
    )?;

    let curr = Local::now().naive_local();
    let exp = curr + Duration::hours(env::var("TOKEN_EXPIRATION_HOURS")?.parse()?);
    let time = exp.timestamp();
    let token = Token {
        content,
        created_at: curr,
        expires: exp
    };

    let (id, content) = diesel::insert_into(tokens::table)
        .values(&token)
        .returning((tokens::id, tokens::content))
        .get_result::<(i32, String)>(&mut data.lock().unwrap().conn)?;
        
    Ok((
        StatusCode::CREATED,
        Json(LoginResponse {
            token: Some(TokenResponse { id, content, expires: time }),
            err: None
        })
    ))
}

pub async fn validate_token(State(AppState { data }): State<AppState>, Json(TokenValidationRequest { id, client }): Json<TokenValidationRequest>) -> Result<AppDataResponse<TokenValidationResponse>, AppError> {
    let token: Option<String> = tokens::table
        .find(id)
        .select(tokens::content)
        .first::<String>(&mut data.lock().unwrap().conn)
        .optional()?;

    match token {
        Some(server) => {
            if !is_valid(TokensEncodeHandler { client, server })? {
                return Ok((
                    StatusCode::UNAUTHORIZED,
                    Json(TokenValidationResponse {
                        validated: false,
                        err: Some("Token is invalid!".to_string())
                    }
                )))
            }

            Ok((
                StatusCode::OK,
                Json(TokenValidationResponse {
                    validated: true,
                    err: None
                }
            )))
        },
        None => Ok((
            StatusCode::NOT_FOUND,
            Json(TokenValidationResponse {
                validated: false,
                err: Some(format!("There are is not any valid token with id {id}"))
            }
        )))
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

                    let result = diesel::delete(FilterDsl::filter(tokens::table, tokens::id.eq(id)))
                        .execute(&mut db.lock().unwrap().conn);

                    if let Err(err) = result {
                        return Err(anyhow!("Could not delete token {id}: {err}")).unwrap()
                    }
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

    if decode_handler.client.claims != decode_handler.server.claims {
        return Ok(false)
    }

    Ok(true)
}

fn handle_decode(token: String) -> anyhow::Result<TokenData<Claims>> {
    let result = decode(
        token.as_str(),
        &DecodingKey::from_secret(env::var("JWT_SECRET")?.as_ref()),
        &Validation::new(Algorithm::default())
    )?;
    Ok(result)
}