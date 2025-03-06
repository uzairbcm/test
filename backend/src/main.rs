mod models;
mod handlers;
mod db;
mod csv;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use handlers::state_handlers::{get_user_state, update_user_state};
use models::app_state::AppState;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use tokio::net::TcpListener;

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
        .route("/api/state/{username}", get(get_user_state))
        .route("/api/state", post(update_user_state))
        .layer(cors)
        .with_state(app_state);
    
    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server running on {}", addr);
    
    // Create a TCP listener and serve the app
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}