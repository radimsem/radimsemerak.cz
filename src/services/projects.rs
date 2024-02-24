use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use axum::Json;
use axum::extract::{State, Multipart};
use axum::http::StatusCode;
use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use serde::Serialize;
use tempfile::NamedTempFile;
use tokio::sync::Mutex;

use crate::{AppState, AppDataResponse};
use crate::error::AppError;
use crate::models::project::Project;
use crate::repository::{db, ActionRequest, ConstructJob, FieldJob, IdentifierAction};
use crate::schema::projects;
use crate::services::parser::MdParser;

#[derive(Default)]
struct ProjectRequest {
    file: Option<NamedTempFile>
}

#[derive(Serialize)]
pub struct ProjectResponse {
    id: i32,
    title: String,
    annotation: String,
    html: String
}

const ANNOTATION_LENGTH: usize = 25;

pub async fn handle_action(State(AppState { db }): State<AppState>, mut multipart: Multipart) -> Result<(StatusCode, ()), AppError> {
    let mut db = db.lock().await;
    let mut expected_fields_with_jobs: HashMap<String, FieldJob<ProjectRequest>> = HashMap::new();

    expected_fields_with_jobs.insert(
        "file".to_string(),
        Arc::new(Mutex::new(Box::new(|body, bytes| {
            let mut file = NamedTempFile::new()?;
            file.write_all(&bytes)?;
            body.file = Some(file);
            Ok(())
        })))
    );
    let action_req: ActionRequest<ProjectRequest> = db::methods::handle_multipart_stream(&mut multipart, &mut expected_fields_with_jobs).await?;

    match action_req.ident_req.action {
        IdentifierAction::UPDATE => {
            let html = validate_file(&action_req.body.file)?;
            match action_req.ident_req.id {
               Some(id) => {
                  diesel::update(FilterDsl::filter(projects::table, projects::id.eq(id)))
                    .set(projects::html.eq(html))
                    .execute(&mut db.conn)?;
               },
               None => {
                  db::methods::insert(
                    projects::table,
                    Project { html },
                    &mut db.conn
                  )?;
               }
            }
        },
        IdentifierAction::DELETE => {
            let id = match action_req.ident_req.id {
                Some(id) => id,
                None => return Err(AppError(
                    anyhow!("Expected an id for delete purposes!"),
                    StatusCode::EXPECTATION_FAILED
                ))
            };
            diesel::delete(FilterDsl::filter(projects::table, projects::id.eq(id)))
                .execute(&mut db.conn)?;
        }
    }

    Ok((StatusCode::OK, ()))
}

pub async fn get_unique(State(AppState { db }): State<AppState>, Json(id): Json<String>) -> Result<AppDataResponse<ProjectResponse>, AppError> {
    let construct_job = get_construct_job();
    let result = db::methods::get_unique(
        projects::table,
        id.parse::<i32>()?,
        construct_job,
        &mut db.lock().await.conn
    ).await?;

    Ok(result)
}

pub async fn get_all(State(AppState { db }): State<AppState>) -> Result<AppDataResponse<Vec<ProjectResponse>>, AppError> {
    let construct_job = get_construct_job();
    let result = db::methods::get_all(
        projects::table,
        construct_job,
        &mut db.lock().await.conn
    ).await?;

    Ok(result)
}

fn get_construct_job() -> ConstructJob<(i32, String), ProjectResponse> {
    Arc::new(Mutex::new(Box::new(|(id, html)| {
        Ok(ProjectResponse {
            id,
            title: get_content("h1", &html)?,
            annotation: get_content("p", &html)?,
            html
        })
    })))
}

fn validate_file(file: &Option<NamedTempFile>) -> Result<String, AppError> {
    match file {
        Some(source) => {
            let file = source.as_file();
            let data = file.metadata()?;
            let path = filename::file_name(source)?;

            if !data.file_type().is_file() {
                return Err(AppError(
                    anyhow!("File does not represent a regular file!"),
                    StatusCode::NOT_ACCEPTABLE
                ))
            }

            if !path.ends_with(".md") {
               return Err(AppError(
                    anyhow!("File is not a Markdown file!"),
                    StatusCode::EXPECTATION_FAILED
               )) 
            }

            let html = MdParser::generate::<PathBuf>(&path)?;
            Ok(html)
        },
        None => Err(AppError(
            anyhow!("File is required for creating or updating purposes!"),
            StatusCode::EXPECTATION_FAILED
        ))
    }
}

fn get_content(tag: &str, html: &String) -> Result<String, AppError> {
    let offset = tag.len() + 2;
    let start = handle_expected_tag(tag, html.find(format!("<{tag}>").as_str()))? + offset;
    let end = handle_expected_tag(tag, html.find(format!("</{tag}>").as_str()))?;
    let mut content = html[start..end].to_string();

    if tag == "p" {
        if content.len() >= ANNOTATION_LENGTH {
            let idx: Option<usize> = content[ANNOTATION_LENGTH..]
                .as_bytes()
                .iter()
                .position(|x: &u8| *x as char == ' ');

            if let Some(val) = idx {
                content = String::from(&content[0..(ANNOTATION_LENGTH + val)]);
            }
        }
        content.push_str("...");
    }

    Ok(content)
}

fn handle_expected_tag(tag: &str, index: Option<usize>) -> Result<usize, AppError> {
    match index {
        Some(idx) => Ok(idx),
        None => Err(AppError(
            anyhow!("Expected tag {tag} not found!"),
            StatusCode::EXPECTATION_FAILED
        ))
    }
} 