use std::sync::{Arc, Mutex};
use std::{env, error::Error};

use axum::{Router, routing::{post, get}};
use dotenv::dotenv;

use semerak::services::projects::{get_projects, get_unique_project, handle_projects_action};
use semerak::AppState;
use semerak::repository::db::Database;
use semerak::services::auth::{handle_tokens_expiration, handle_login_auth, verify_token};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let port = env::var("PORT")?;
    let db = Database::build()?;
    println!("Connected to database");

    let state = AppState { data: Arc::new(Mutex::new(db)) };
    let app: Router = Router::new()
        .route("/api/login", post(handle_login_auth))
        .route("/api/verify", post(verify_token))
        .route("/api/projects/action", post(handle_projects_action))
        .route("/api/projects/unique", post(get_unique_project))
        .route("/api/projects/all", get(get_projects))
        .route("/api/expires", get(handle_tokens_expiration))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind(
        format!("127.0.0.1:{port}"))
        .await?;
    
    println!("Server started on 127.0.0.1:{port}");
    axum::serve(listener, app).await?;

    Ok(())
}