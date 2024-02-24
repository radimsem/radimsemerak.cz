use std::env;
use diesel::{Connection, PgConnection};
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
}

pub mod methods {
    use diesel::PgConnection;
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

    pub async fn handle_multipart_stream<T: Default>(multipart: &mut Multipart, expected_fields_with_jobs: &mut HashMap<String, FieldJob<T>>) -> Result<ActionRequest<T>, AppError> {
        let mut action_req = ActionRequest {
            body: T::default(),
            ident_req: IdentifierRequest::default()
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
                let bytes = field.bytes().await?.to_vec();
                match expected_fields_with_jobs.remove(&name) {
                    Some(job) => {
                        let job = job.lock().await;
                        job(&mut action_req.body, &bytes)?;
                    },
                    None => return Err(AppError(
                        anyhow!("Expected field does not have a job!"),
                        StatusCode::EXPECTATION_FAILED
                    ))
                }
            } else {
                let text = field.text().await?;
                match name.as_str() {
                    "id" => action_req.ident_req.id = Some(text.as_str().parse()?),
                    "action" => action_req.ident_req.action = match text.as_str() {
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
        Ok(action_req) 
    }
    
    pub fn insert<T, U>(table: T, records: U, conn: &mut PgConnection) -> anyhow::Result<()>
    where
        T: Table,
        U: Insertable<T>,
        InsertStatement<T, <U as Insertable<T>>::Values>: ExecuteDsl<PgConnection>
    {
        diesel::insert_into(table)
            .values(records)
            .execute(conn)?;
        Ok(()) 
    }

    pub async fn get_unique<'a, T, U, V, PK>(table: T, id: PK, construct_job: ConstructJob<U, V>, conn: &mut PgConnection) -> Result<AppDataResponse<V>, AppError>
    where
        T: FindDsl<PK>,
        <T as FindDsl<PK>>::Output: LoadQuery<'a, PgConnection, U>
    {
        let construct_job = construct_job.lock().await;
        let result: U = table
            .find(id)
            .load::<U>(conn)?
            .remove(usize::default());

        Ok((
            StatusCode::OK,
            Json(construct_job(result)?)
        ))
    }

    pub async fn get_all<'a, T, U, V>(table: T, construct_job: ConstructJob<U, V>, conn: &mut PgConnection) -> Result<AppDataResponse<Vec<V>>, AppError>
    where
        T: RunQueryDsl<Pg>,
        T: LoadQuery<'a, PgConnection, U>,
    {
        let construct_job = construct_job.lock().await;
        let results: Vec<Result<V, AppError>> = table
           .load::<U>(conn)?
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