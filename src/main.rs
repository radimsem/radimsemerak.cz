use std::env;
use std::error::Error;
use std::sync::Arc;

use axum::Router;
use axum::routing::{post, get};
use dotenv::dotenv;
use tokio::sync::Mutex;

use semerak::AppState;
use semerak::repository::db::Database;
use semerak::services::auth;
use semerak::services::projects;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let port = env::var("PORT")?;
    let db = Database::build()?;
    println!("Connected to database");

    let state = AppState { db: Arc::new(Mutex::new(db)) };
    let app: Router = Router::new()
        .route("/api/login", post(auth::handle_login_auth))
        .route("/api/verify", post(auth::verify_token))
        .route("/api/projects/action", post(projects::handle_action))
        .route("/api/projects/unique", post(projects::get_unique))
        .route("/api/projects/all", get(projects::get_all))
        .route("/api/expires", get(auth::handle_tokens_expiration))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind(
        format!("127.0.0.1:{port}"))
        .await?;
    
    println!("Server started on 127.0.0.1:{port}");
    axum::serve(listener, app).await?;

    Ok(())
}