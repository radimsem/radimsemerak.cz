use std::env;
use std::collections::HashMap;

use anyhow::anyhow;
use axum::Json;
use axum::http::StatusCode;
use axum::extract::{multipart::Field, Multipart};
use diesel::{Connection, PgConnection, QueryableByName, RunQueryDsl};
use diesel::query_dsl::methods::{FindDsl, LoadQuery};
use diesel::pg::Pg;

use crate::AppDataResponse;
use crate::repository::{is_default, complete_db_uri};
use crate::error::AppError;
use crate::services::projects::{IdentifierRequest, IdentifierAction};

pub struct Database {
    pub conn: PgConnection
}

pub struct ActionRequest<T: Default> {
    body: T,
    idr: IdentifierRequest 
}

impl Database {
    pub fn build() -> anyhow::Result<Self> {
        let db_uri = complete_db_uri(&mut env::var("DB_URI")?, env::var("DB_PASSWORD")?)?;
        let conn = PgConnection::establish(&db_uri)?;

        Ok(Self { conn })
    }

    pub async fn handle_stream<T: Default>(mut multipart: Multipart, expected_fields_with_jobs: HashMap<String, impl Fn(&mut T, &Field<'_>)>) -> Result<ActionRequest<T>, AppError> {
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
              match expected_fields_with_jobs.get(&name) {
                  Some(job) => job(&mut acr.body, &field),
                  None => return Err(AppError(
                    anyhow!("Expected field does not have a job!"),
                    StatusCode::EXPECTATION_FAILED
                  ))
              }
           } else {
              let value = field.text().await?;
              match name.as_str() {
                 "id" => acr.idr.id = value.as_str().parse()?,
                 "action" => acr.idr.action = match value.as_str() {
                    "update" => IdentifierAction::UPDATE,
                    "delete" => IdentifierAction::DELETE,
                    _ => return Err(AppError(
                        anyhow!("Unexpected action {value}"),
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

    pub fn handle_action<T: Default>(&self, acr: ActionRequest<T>) -> anyhow::Result<()> { 
        if !is_default(&acr.idr) {
           match acr.idr.action {
               IdentifierAction::UPDATE => {
                  //TODO Generic update
               },
               IdentifierAction::DELETE => {
                  //TODO Generic delete
               }
           } 
        } else {
            //TODO Generic insert
        }
        
        Ok(())
    }

    pub fn get_unique<'a, T, U>(&mut self, table: T, id: i32, construct_job: impl Fn(U) -> Result<U, AppError>) -> Result<AppDataResponse<U>, AppError>
    where
        T: FindDsl<i32>,
        <T as FindDsl<i32>>::Output: LoadQuery<'a, PgConnection, U>,
        U: QueryableByName<Pg>
    {
        let result: U = table
            .find(id)
            .load(&mut self.conn)?
            .remove(usize::default());

        Ok((
            StatusCode::OK,
            Json(construct_job(result)?)
        ))
    }

    pub fn get_all<'a, T, U, V>(&mut self, table: T, construct_job: impl Fn(U) -> Result<V, AppError>) -> Result<AppDataResponse<Vec<V>>, AppError>
    where
        T: RunQueryDsl<Pg>,
        T: LoadQuery<'a, PgConnection, U>,
        U: QueryableByName<Pg>
    {
        let results: Vec<Result<V, AppError>> = table.load::<U>(&mut self.conn)?
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