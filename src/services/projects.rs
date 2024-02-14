use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use axum::Json;
use axum::extract::{State, Multipart};
use axum::http::StatusCode;
use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use serde::Serialize;
use tempfile::NamedTempFile;

use crate::{AppState, AppDataResponse};
use crate::error::AppError;
use crate::models::project::Project;
use crate::repository::db::Database;
use crate::schema::projects;
use crate::services::parser::MdParser;

#[derive(Default)]
struct DataRequest {
    idn: Option<IdentifierRequest>,
    file: Option<NamedTempFile>,
}

#[derive(Default, PartialEq)]
pub struct IdentifierRequest {
    pub id: i32,
    pub action: IdentifierAction
}

#[derive(Default, PartialEq)]
pub enum IdentifierAction {
    #[default]
    UPDATE,
    DELETE
}

#[derive(Serialize)]
pub struct ProjectResponse {
    id: i32,
    title: String,
    annotation: String,
    html: String
}

const ANNOTATION_LENGTH: usize = 25;

pub async fn handle_projects_action(State(AppState { db }): State<AppState>, mut multipart: Multipart) -> Result<(StatusCode, ()), AppError> {
    let mut req = DataRequest::default();
    let mut idn = IdentifierRequest::default();

    while let Some(field) = multipart.next_field().await? {
        let name = match field.name() {
            Some(name) => name.to_string(),
            None => return Err(AppError(
                anyhow!("Field's name is required!"),
                StatusCode::EXPECTATION_FAILED
            ))
        };

        match name.as_str() {
            "file" => {
                let bytes = field.bytes().await?;
                let mut file = NamedTempFile::new()?;
                file.write_all(&bytes)?;
                req.file = Some(file);
            },
            _ => {
                let value = field.text().await?;
                match name.as_str() {
                    "id" => idn.id = value.as_str().parse()?,
                    "action" => idn.action = match value.as_str() {
                        "update" => IdentifierAction::UPDATE,
                        "delete" => IdentifierAction::DELETE,
                        _ => return Err(AppError(
                            anyhow!("Unexpected action {value}"),
                            StatusCode::EXPECTATION_FAILED
                        ))
                    },
                    _ => return Err(AppError(
                        anyhow!("Unexpected field with name {name}"),
                        StatusCode::EXPECTATION_FAILED
                    ))
                }
            }
        }
    }

    if idn != IdentifierRequest::default() {
        req.idn = Some(idn);
    }
    handle_action(&db, req)?;

    Ok((StatusCode::OK, ()))
}

pub async fn get_projects(State(AppState { db }): State<AppState>) -> Result<AppDataResponse<Vec<ProjectResponse>>, AppError> {
    let results: Vec<Result<ProjectResponse, AppError>> = projects::table
        .select((projects::id, projects::html))
        .load::<(i32, String)>(&mut db.lock().unwrap().conn)?
        .into_iter()
        .map(|(id, html)| Ok( 
            ProjectResponse { 
                id, 
                title: get_content("h1", &html)?,
                annotation: get_content("p", &html)?,
                html 
            })
        )
        .collect();

    let mut projects: Vec<ProjectResponse> = Vec::with_capacity(results.capacity());
    for result in results { projects.push(result?); }
    
    Ok((
        StatusCode::OK,
        Json(projects)
    ))
}

pub async fn get_unique_project(State(AppState { db }): State<AppState>, Json(id): Json<String> ) -> Result<AppDataResponse<ProjectResponse>, AppError> {
    let result: Option<String> = projects::table
        .find(id.parse::<i32>()?)
        .select(projects::html)
        .first::<String>(&mut db.lock().unwrap().conn)
        .optional()?;

    match result {
        Some(html) => Ok((
            StatusCode::OK,
            Json(ProjectResponse {
                id: id.as_str().parse()?,
                title: get_content("h1", &html)?,
                annotation: get_content("p", &html)?,
                html
            })
        )),
        None => Err(AppError(
            anyhow!("Unable to find project with id: {id}"),
            StatusCode::NOT_FOUND
        ))
        
    }
}

fn handle_action(db: &Arc<Mutex<Database>>, body: DataRequest) -> Result<(), AppError> {
    let conn: &mut PgConnection = &mut db.lock().unwrap().conn;
    match body.idn {
        Some(idn) => match idn.action {
            IdentifierAction::UPDATE => {
                let html = validate_file(&body.file)?;
                diesel::update(FilterDsl::filter(projects::table, projects::id.eq(idn.id)))
                    .set(projects::html.eq(html))
                    .execute(conn)?;
            },
            IdentifierAction::DELETE => {
                diesel::delete(FilterDsl::filter(projects::table, projects::id.eq(idn.id)))
                    .execute(conn)?;
            }
        },
        None => {
            let html = validate_file(&body.file)?;
            diesel::insert_into(projects::table)
                .values(&Project { html })
                .execute(conn)?;
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
            let html = MdParser::generate::<PathBuf>(&path)?;

            Ok(html)
        },
        None => return Err(AppError(
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
        if content.len() > ANNOTATION_LENGTH {
            let idx: Option<usize> = content.as_bytes()[ANNOTATION_LENGTH..].iter().position(|x: &u8| *x as char == ' '); 
            if let Some(val) = idx {
                content = String::from(&content[0..val]);
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