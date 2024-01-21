mod models;
mod repository;
mod services;
mod error;
mod schema;

use std::sync::{Arc, Mutex};
use std::{env, error::Error};

use axum::{Router, routing::{post, get}};
use dotenv::dotenv;

use crate::repository::db::Database;
use crate::services::auth::{login_auth_handler, validate_token, handle_tokens_expiration};

#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<Database>>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    let port = env::var("PORT")?;
    let db = Database::build()?;
    println!("Connected to database");

    let state = AppState { data: Arc::new(Mutex::new(db)) };
    let app: Router = Router::new()
        .route("/api/login", post(login_auth_handler))
        .route("/api/validate", post(validate_token))
        .route("/expire", get(handle_tokens_expiration))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind(
        format!("127.0.0.1:{port}"))
        .await?;
    
    println!("Server started on 127.0.0.1:{port}");
    axum::serve(listener, app).await?;

    Ok(())
}