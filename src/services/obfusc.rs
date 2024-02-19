use anyhow::anyhow;
use axum::Json;
use axum::http::StatusCode;
use serde::Deserialize;

use crate::AppDataResponse;
use crate::error::AppError;

#[derive(Deserialize)]
enum Job {
    ENCODE,
    DECODE
}

#[derive(Deserialize)]
pub struct ObfuscRequest {
    content: String,
    job: Job
}

const DEFAULT_DIGIT_BASE: u32 = 10;

pub async fn handle_obfuscation(Json(ObfuscRequest { content, job }): Json<ObfuscRequest>) -> Result<AppDataResponse<String>, AppError> {
    let mut target = String::new();

    match job {
        Job::ENCODE => {
            for x in content.chars().into_iter() {
                let digit = match x.to_digit(DEFAULT_DIGIT_BASE) {
                    Some(val) => val,
                    None => return Err(AppError(
                        anyhow!("Unable to convert char {} into digit with base {}!", x, DEFAULT_DIGIT_BASE),
                        StatusCode::EXPECTATION_FAILED
                    ))
                };
                let result = format!("&#{};", digit);
                target.push_str(result.as_str());
            }
        },
        Job::DECODE => {
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
                let result = match char::from_digit(digit, DEFAULT_DIGIT_BASE) {
                    Some(val) => val,
                    None => return Err(AppError(
                        anyhow!("Unable to convert digit {} into char with base {}!", digit, DEFAULT_DIGIT_BASE),
                        StatusCode::EXPECTATION_FAILED
                    ))
                };
                target.push(result);
            }
        } 
    };

    Ok((StatusCode::OK, Json(target)))
}