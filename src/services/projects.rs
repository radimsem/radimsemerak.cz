use std::{fs::File, path::PathBuf};

use anyhow::anyhow;
use axum::{extract::State, http::StatusCode, Json};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};

use crate::{error::AppError, models::project::Project, schema::projects, AppState};

use super::parser::MdParser;

pub struct Request {
    id: Option<IdentifierRequest>,
    file: Option<File>
}

struct IdentifierRequest {
    id: i32,
    job: IdentifierJob
}

enum IdentifierJob {
    UPDATE,
    DELETE
}

async fn projects_handler(State(AppState { data }): State<AppState>, Json(req): Json<Request>) -> Result<(StatusCode, ()), AppError> {
    match req.id {
        Some(idn) => match idn.job {
            IdentifierJob::UPDATE => {
                let html = validate_file(&req.file)?;
                diesel::update(projects::table.filter(projects::id.eq(idn.id)))
                    .set(projects::html.eq(html))
                    .execute(&mut data.lock().unwrap().conn)?;
            },
            IdentifierJob::DELETE => {
                diesel::delete(FilterDsl::filter(projects::table, projects::id.eq(idn.id)))
                    .execute(&mut data.lock().unwrap().conn)?;
            }
        },
        None => {
            let html = validate_file(&req.file)?;
            diesel::insert_into(projects::table)
                .values(&Project { html })
                .execute(&mut data.lock().unwrap().conn)?;
        }
    }

    Ok((StatusCode::OK, ()))
}

fn validate_file(file: &Option<File>) -> Result<String, AppError> {
    match file {
        Some(source) => {
            let data = source.metadata()?;
            let path = filename::file_name(source)?;

            if !data.file_type().is_file() {
                return Err(AppError(
                    anyhow!("File does not represent a regular file!"),
                    StatusCode::NOT_ACCEPTABLE
                ))
            }
            match path.extension() {
                Some(ext) => {
                    if ext != "md" {
                        return Err(AppError(
                            anyhow!("File's extension {:?} is not 'md'!", ext),
                            StatusCode::NOT_ACCEPTABLE
                        ))
                    }
                },
                None => return Err(AppError(
                    anyhow!("File has no extension!"),
                    StatusCode::NOT_ACCEPTABLE
                ))
            }

            let html = MdParser::generate::<PathBuf>(&path)?;

            Ok(html)
        },
        None => return Err(AppError(
            anyhow!("File is required for creating or updating purposes!"),
            StatusCode::EXPECTATION_FAILED
        ))
    }
}