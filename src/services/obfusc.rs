use anyhow::anyhow;
use axum::Json;
use axum::http::StatusCode;
use serde::Deserialize;

use crate::AppDataResponse;
use crate::error::AppError;

#[derive(Deserialize)]
pub struct ObfuscRequest {
    content: String,
    job: String
}

const RADIX: u32 = 10;

pub async fn handle_obfuscation(Json(ObfuscRequest { content, job }): Json<ObfuscRequest>) -> Result<AppDataResponse<String>, AppError> {
    let mut target = String::new();

    match job.as_str() {
        "encode" => {
            for x in content.chars().into_iter() {
                let encode = format!("&#{};", x as u32);
                target.push_str(encode.as_str());
            }
        },
        "decode" => {
            let encodes = content.split(';');
            for encode in encodes {
                let offset = match encode.find('#') {
                   Some(idx) => idx,
                   None => return Err(AppError(
                        anyhow!("Encoded content does not to have '#' char!"),
                        StatusCode::EXPECTATION_FAILED
                   )) 
                } + 1;
                let digit: u32 = encode[offset..].parse()?;
                let result = match char::from_digit(digit, RADIX) {
                    Some(val) => val,
                    None => return Err(AppError(
                        anyhow!("Unable to convert digit {} into char with base {}!", digit, RADIX),
                        StatusCode::EXPECTATION_FAILED
                    ))
                };
                target.push(result);
            }
        },
        _ => return Err(AppError(
            anyhow!("Unexpected job {}!", job),
            StatusCode::EXPECTATION_FAILED
        )) 
    };

    Ok((StatusCode::OK, Json(target)))
}