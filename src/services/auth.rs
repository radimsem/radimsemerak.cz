use std::env;

use anyhow::anyhow;
use async_io::Timer;
use axum::extract::State;
use axum::{Json, http::StatusCode};
use chrono::{Local, Duration, NaiveDateTime};
use diesel::{ExpressionMethods, RunQueryDsl, prelude::*};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::models::token::Token;
use crate::error::AppError;
use crate::schema::tokens;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    pw: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: Option<String>,
    err: Option<String>
}

#[derive(Serialize)]
pub struct TokenValidationResponse {
    validated: bool,
    err: Option<String>
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    company: String,
}

pub async fn login_auth_handler(State(AppState { data }): State<AppState>, Json(LoginRequest { username, pw }): Json<LoginRequest>) -> Result<(StatusCode, Json<LoginResponse>), AppError> {
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
    let token = Token {
        content,
        created_at: curr,
        expires: curr + Duration::hours(env::var("TOKEN_EXPIRATION_HOURS")?.parse()?)
    };

    let content = diesel::insert_into(tokens::table)
        .values(&token)
        .returning(tokens::content)
        .get_result::<String>(&mut data.lock().unwrap().conn)?;
        
    Ok((
        StatusCode::CREATED,
        Json(LoginResponse {
            token: Some(content),
            err: None
        })
    ))
}

pub async fn validate_token(State(AppState { data }): State<AppState>, Json(token_content): Json<String>) -> Result<(StatusCode, Json<TokenValidationResponse>), AppError> {
    let result: Vec<String> = tokens::table.select(tokens::content)
        .filter(tokens::content.eq(token_content))
        .load(&mut data.lock().unwrap().conn)?;

    match result.get(usize::default()) {
        Some(_) => Ok((
            StatusCode::OK,
            Json(TokenValidationResponse {
                validated: true,
                err: None
            }
        ))),
        None => Ok((
            StatusCode::NOT_FOUND,
            Json(TokenValidationResponse {
                validated: false,
                err: Some("There is no valid token at the moment!".to_string())
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

                    let result = diesel::delete(tokens::table.filter(tokens::id.eq(id)))
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