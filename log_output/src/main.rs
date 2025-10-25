use axum::{Router, routing::get};
use chrono::Local;
use std::env;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use tokio::net::TcpListener;
use tracing::{error, info};
use uuid::Uuid;

async fn timestamp() -> String {
    let now = Local::now();
    let random_id = Uuid::new_v4();
    format!("{}: {}", now.format("%Y-%m-%dT%H:%M:%SZ"), random_id)
}

async fn handle_root(file_path: &str) -> String {
    match std::fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            error!("Failed to read log file: {}", e);
            "Error reading log file".to_string()
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // reader or writer mode
    let application_type = env::var("APPLICATION_TYPE").unwrap_or_else(|_| "reader".to_string());
    let port = "5678";
    let log_path = "/opt/logger/output.log";

    if application_type == "writer" {
        info!("Starting in writer mode");

        loop {
            let current_timestamp = timestamp().await;
            match std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path)
            {
                Ok(mut f) => {
                    if let Err(e) = writeln!(f, "{}", current_timestamp) {
                        error!("Failed to write to log file: {}", e);
                    } else {
                        info!("Wrote timestamp to log file: {}", current_timestamp);
                    }
                }
                Err(e) => {
                    error!("Failed to open log file: {}", e);
                }
            }
            sleep(Duration::new(5, 0));
        }
    } else {
        info!("Starting in reader mode");

        let app = Router::new().route("/", get(move || async move {
            handle_root(log_path).await
        }));

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
}
