use chrono::Local;
use uuid::Uuid;
use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tracing::{info, error};

async fn timestamp() -> String {
    let now = Local::now();
    let random_id = Uuid::new_v4();
    format!("{}: {}", now.format("%Y-%m-%dT%H:%M:%SZ"), random_id)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = "5678";
    let app = Router::new()
        .route("/", get(timestamp));

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
