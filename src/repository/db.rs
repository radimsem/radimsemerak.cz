use std::env;
use diesel::{Connection, PgConnection};
use std::collections::HashMap;

use anyhow::anyhow;
use axum::extract::Multipart;
use axum::Json;
use axum::http::StatusCode;
use diesel::query_builder::InsertStatement;
use diesel::{Insertable, RunQueryDsl, Table};
use diesel::query_dsl::methods::{ExecuteDsl, FindDsl, LoadQuery};
use diesel::pg::Pg;

use crate::AppDataResponse;
use crate::error::AppError;
use crate::repository::{ActionRequest, IdentifierRequest, IdentifierAction, FieldJob, ConstructJob};
use crate::repository::complete_db_uri;

pub struct Database {
    pub conn: PgConnection
}

impl Database {
    pub fn build() -> anyhow::Result<Self> {
        let db_uri = complete_db_uri(&mut env::var("DB_URI")?, env::var("DB_PASSWORD")?)?;
        let conn = PgConnection::establish(&db_uri)?;

        Ok(Self { conn })
    }

    pub async fn handle_multipart_stream<T: Default>(&self, multipart: &mut Multipart, expected_fields_with_jobs: &mut HashMap<String, FieldJob<T>>) -> Result<ActionRequest<T>, AppError> {
        let mut acr = ActionRequest {
            body: T::default(),
            idr: IdentifierRequest::default()
        };

        while let Some(field) = multipart.next_field().await? {
            let name = match field.name() {
                Some(val) => val.to_string(),
                None => return Err(AppError(
                    anyhow!("Field's name is required!"),
                    StatusCode::EXPECTATION_FAILED
                )) 
            };

            if expected_fields_with_jobs.contains_key(&name) {
                let bytes = field.bytes().await?;
                match expected_fields_with_jobs.remove(&name) {
                    Some(job) => job(&mut acr.body, &bytes)?,
                    None => return Err(AppError(
                        anyhow!("Expected field does not have a job!"),
                        StatusCode::EXPECTATION_FAILED
                    ))
                }
            } else {
                let text = field.text().await?;
                match name.as_str() {
                    "id" => acr.idr.id = Some(text.as_str().parse()?),
                    "action" => acr.idr.action = match text.as_str() {
                        "update" => IdentifierAction::UPDATE,
                        "delete" => IdentifierAction::DELETE,
                        _ => return Err(AppError(
                            anyhow!("Unexpected action {text}"),
                            StatusCode::EXPECTATION_FAILED
                        ))
                    },
                    _ => return Err(AppError(
                        anyhow!("Unexpected field with name {name}!"),
                        StatusCode::EXPECTATION_FAILED
                    ))
                }
            }
        }
        Ok(acr) 
    }
    
    pub fn insert<T, U>(&mut self, table: T, records: U) -> anyhow::Result<()>
    where
        T: Table,
        U: Insertable<T>,
        InsertStatement<T, <U as Insertable<T>>::Values>: ExecuteDsl<PgConnection>
    {
        diesel::insert_into(table)
            .values(records)
            .execute(&mut self.conn)?;
        Ok(()) 
    }

    pub fn get_unique<'a, T, U, V, PK>(&mut self, table: T, id: PK, construct_job: ConstructJob<U, V>) -> Result<AppDataResponse<V>, AppError>
    where
        T: FindDsl<PK>,
        <T as FindDsl<PK>>::Output: LoadQuery<'a, PgConnection, U>
    {
        let result: U = table
            .find(id)
            .load::<U>(&mut self.conn)?
            .remove(usize::default());

        Ok((
            StatusCode::OK,
            Json(construct_job(result)?)
        ))
    }

    pub fn get_all<'a, T, U, V>(&mut self, table: T, construct_job: ConstructJob<U, V>) -> Result<AppDataResponse<Vec<V>>, AppError>
    where
        T: RunQueryDsl<Pg>,
        T: LoadQuery<'a, PgConnection, U>,
    {
        let results: Vec<Result<V, AppError>> = table
           .load::<U>(&mut self.conn)?
           .into_iter()
           .map(|item: U| construct_job(item))
           .collect();

        let mut items: Vec<V> = Vec::with_capacity(results.capacity());
        for result in results {
           items.push(result?); 
        }

        Ok((
            StatusCode::OK,
            Json(items)
        ))
    }
}