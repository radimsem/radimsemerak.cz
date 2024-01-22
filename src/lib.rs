use std::sync::{Arc, Mutex};
use repository::db::Database;

pub mod models;
pub mod repository;
pub mod services;
pub mod error;
pub mod schema;

#[derive(Clone)]
pub struct AppState {
    pub data: Arc<Mutex<Database>>
}