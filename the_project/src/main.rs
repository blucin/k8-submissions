use axum::{
    response::Html,
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use std::env;
use tracing::{info, error};

async fn home() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let app = Router::new()
        .route("/", get(home));

    let listener = match TcpListener::bind(format!("0.0.0.0:{}", port)).await {
        Ok(l) => {
            info!("Server started in port {}", port);
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
