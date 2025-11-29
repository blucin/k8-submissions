use axum::extract::State;
use axum::{Router, routing::get};
use std::env;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::net::TcpListener;
use tracing::{error, info};

const FILE_PATH: &str = "/opt/logger/output.log";

async fn pinger(State(counter): State<Arc<AtomicUsize>>) -> String {
    let n: usize = counter.fetch_add(1, Ordering::SeqCst);
    let data = format!("Ping / Pongs: {}", n);
    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_PATH)
    {
        Ok(mut f) => {
            if let Err(e) = writeln!(f, "{}", data) {
                error!("Failed to write to log file: {}", e);
                "Error writing log file".to_string()
            } else {
                data
            }
        }
        Err(e) => {
            error!("Failed to open log file: {}", e);
            "Error writing log file".to_string()
        }
    }
}

async fn pings(State(counter): State<Arc<AtomicUsize>>) -> String {
    let n: usize = counter.load(Ordering::SeqCst);
    format!("Pings / Pongs: {}", n)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let counter = Arc::new(AtomicUsize::new(0));
    let app = Router::new()
        .route("/pingpong", get(pinger))
        .route("/pings", get(pings))
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
