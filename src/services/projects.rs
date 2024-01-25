use std::{io::Write, path::PathBuf, sync::{Arc, Mutex}};

use anyhow::anyhow;
use axum::{extract::{State, Multipart}, http::StatusCode, Json};
use diesel::{query_dsl::methods::{FilterDsl, SelectDsl}, ExpressionMethods, RunQueryDsl};
use serde::Serialize;
use tempfile::NamedTempFile;

use crate::{error::AppError, models::project::Project, repository::db::Database, schema::projects, AppDataResponse, AppState};
use crate::services::parser::MdParser;

#[derive(Default)]
struct DataRequest {
    idn: Option<IdentifierRequest>,
    file: Option<NamedTempFile>,
}

#[derive(Default, PartialEq)]
struct IdentifierRequest {
    id: i32,
    action: IdentifierAction
}

#[derive(Default, PartialEq)]
enum IdentifierAction {
    #[default]
    UPDATE,
    DELETE
}

#[derive(Serialize)]
pub struct ProjectResponse {
    id: i32,
    html: String
}

impl From<(i32, String)> for ProjectResponse {
    fn from((id, html): (i32, String)) -> Self {
        Self { id, html }
    }
}

pub async fn handle_projects_action(State(AppState { data }): State<AppState>, mut multipart: Multipart) -> Result<(StatusCode, ()), AppError> {
    let mut req = DataRequest::default();
    let mut idn = IdentifierRequest::default();

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or_else(|| "").to_string();
        let bytes = field.bytes().await?;

        match name.as_str() {
            "file" => {
                let mut file = NamedTempFile::new()?;
                file.write_all(&bytes)?;
                req.file = Some(file);
            },
            "id" => idn.id = serde_json::from_slice::<i32>(&bytes)?,
            "action" => {
                let action = serde_json::from_slice::<String>(&bytes)?;
                idn.action = match action.as_str() {
                    "update" => IdentifierAction::UPDATE,
                    "delete" => IdentifierAction::DELETE,
                    &_ => return Err(AppError(
                        anyhow!("Unexpected action {action}!"),
                        StatusCode::EXPECTATION_FAILED
                    ))
                }
            },
            &_ => return Err(AppError(
                anyhow!("Unexpected field with name {name}!"),
                StatusCode::EXPECTATION_FAILED
            ))
        }
    }

    if idn != IdentifierRequest::default() {
        req.idn = Some(idn);
    }
    handle_action(&data, req)?;

    Ok((StatusCode::OK, ()))
}

pub async fn get_projects(State(AppState { data }): State<AppState>) -> Result<AppDataResponse<Vec<ProjectResponse>>, AppError> {
    let projects: Vec<ProjectResponse> = projects::table
        .select((projects::id, projects::html))
        .load::<(i32, String)>(&mut data.lock().unwrap().conn)?
        .into_iter()
        .map(|(id, html)| ProjectResponse { id, html })
        .collect();
    
    Ok((
        StatusCode::OK,
        Json(projects)
    ))
}

fn handle_action(data: &Arc<Mutex<Database>>, body: DataRequest) -> Result<(), AppError> {
    match body.idn {
        Some(idn) => match idn.action {
            IdentifierAction::UPDATE => {
                let html = validate_file(&body.file)?;
                diesel::update(projects::table.filter(projects::id.eq(idn.id)))
                    .set(projects::html.eq(html))
                    .execute(&mut data.lock().unwrap().conn)?;
            },
            IdentifierAction::DELETE => {
                diesel::delete(FilterDsl::filter(projects::table, projects::id.eq(idn.id)))
                    .execute(&mut data.lock().unwrap().conn)?;
            }
        },
        None => {
            let html = validate_file(&body.file)?;
            diesel::insert_into(projects::table)
                .values(&Project { html })
                .execute(&mut data.lock().unwrap().conn)?;
        }
    }

    Ok(())
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
            match path.extension() {
                Some(ext) => {
                    if ext != "md" {
                        return Err(AppError(
                            anyhow!("File's extension {:?} is not a Markdown file!", ext),
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