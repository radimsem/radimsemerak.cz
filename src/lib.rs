use std::sync::Arc;
use axum::Json;
use axum::http::StatusCode;
use repository::db::Database;
use tokio::sync::Mutex;

pub mod models;
pub mod repository;
pub mod services;
pub mod error;
pub mod schema;

pub type AppDataResponse<T> = (StatusCode, Json<T>);

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Database>>
}