use std::env;

use anyhow::anyhow;
use async_io::Timer;
use axum::extract::State;
use axum::{Json, http::StatusCode};
use chrono::{Local, Duration, NaiveDateTime};
use diesel::{ExpressionMethods, RunQueryDsl, prelude::*};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey, decode, DecodingKey, Validation, TokenData};
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
    token: Option<TokenResponse>,
    err: Option<String>
}

#[derive(Serialize)]
pub struct TokenResponse {
    content: String,
    expires: i64
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
    let exp = curr + Duration::hours(env::var("TOKEN_EXPIRATION_HOURS")?.parse()?);
    let time = exp.timestamp();
    let token = Token {
        content,
        created_at: curr,
        expires: exp
    };

    let content = diesel::insert_into(tokens::table)
        .values(&token)
        .returning(tokens::content)
        .get_result::<String>(&mut data.lock().unwrap().conn)?;
        
    Ok((
        StatusCode::CREATED,
        Json(LoginResponse {
            token: Some(TokenResponse { content, expires: time }),
            err: None
        })
    ))
}

pub async fn validate_token(State(AppState { data }): State<AppState>, Json(client): Json<String>) -> Result<(StatusCode, Json<TokenValidationResponse>), AppError> {
    let mut tokens: Vec<String> = tokens::table.select(tokens::content)
        .load::<String>(&mut data.lock().unwrap().conn)?;
    
    if tokens.len() == 0 {
        return Ok((
            StatusCode::NOT_FOUND,
            Json(TokenValidationResponse {
                validated: false,
                err: Some("There is no valid token at the moment!".to_string())
            }
        )))
    }

    tokens.push(client);
    let mut decodes: Vec<TokenData<Claims>> = Vec::with_capacity(tokens.capacity());
    for token in tokens {
        decodes.push(
            decode::<Claims>(
                token.as_str(), 
                &DecodingKey::from_secret(env::var("JWT_SECRET")?.as_ref()), 
                &Validation::new(Algorithm::default())
            )?
        );
    }

    let len = decodes.len();
    if !decodes[0..len - 2].iter().any(|token| token.claims == decodes[len - 1].claims) {
        return Ok((
            StatusCode::UNAUTHORIZED,
            Json(TokenValidationResponse {
                validated: false,
                err: Some("The token does not match any other on the server!".to_string())
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