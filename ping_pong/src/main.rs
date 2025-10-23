use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use std::env;
use tracing::{info, error};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use axum::extract::State;

async fn pinger(State(counter): State<Arc<AtomicUsize>>) -> String {
    let n: usize = counter.fetch_add(1, Ordering::SeqCst);
    format!("pong {}", n)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let counter = Arc::new(AtomicUsize::new(0));
    let app = Router::new()
        .route("/pingpong", get(pinger))
        .with_state(counter);


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

