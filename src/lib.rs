use std::sync::{Arc, Mutex};
use axum::{http::StatusCode, Json};
use repository::db::Database;

pub mod models;
pub mod repository;
pub mod services;
pub mod error;
pub mod schema;

pub type AppDataResponse<T> = (StatusCode, Json<T>);

#[derive(Clone)]
pub struct AppState {
    pub data: Arc<Mutex<Database>>
}