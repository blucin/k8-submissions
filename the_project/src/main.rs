use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use std::env;
use tracing::{info, error};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let app = Router::new()
        .route("/", get(|| async { "Hello world" }));

    let listener = match TcpListener::bind(format!("localhost:{}", port)).await {
        Ok(l) => {
            info!("Listening on port {}", port);
            l
        }
        Err(e) => {
            error!("Failed to bind to port {}: {}", port, e);
            return;
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        error!("Server error: {}", e);
    }
}
