mod models;
mod handlers;
mod db;
mod csv;

use axum::{
    extract::{Path, State},
    http::{HeaderValue, Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use handlers::state_handlers::{get_user_state, update_user_state};
use models::app_state::AppState;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Initialize app state
    let app_state = Arc::new(AppState::new().await?);
    
    // Define CORS middleware
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);
    
    // Define routes
    let app = Router::new()
        .route("/api/state/:username", get(get_user_state))
        .route("/api/state", post(update_user_state))
        .layer(cors)
        .with_state(app_state);
    
    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}