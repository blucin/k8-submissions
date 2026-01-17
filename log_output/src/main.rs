use axum::{Router, routing::get};
use chrono::Local;
use std::env;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use tokio::net::TcpListener;
use tracing::{error, info};
use uuid::Uuid;
use reqwest::Client;

const PING_PONG_URL: &str = "http://ping-pong-service:3000/pings";

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

async fn handle_normal() -> String {
    let timestamp_str = timestamp().await;
    let message = env::var("MESSAGE").unwrap_or_else(|_| "".to_string());
    let mut output = String::new();
    
    if let Ok(file_content) = std::fs::read_to_string("/config/information.txt") {
        output.push_str(&format!("file content: {}\n", file_content.trim()));
    } else {
        error!("Failed to read /config/information.txt");
    }
    
    if !message.is_empty() {
        output.push_str(&format!("env variable: MESSAGE={}\n", message));
    } else {
        error!("MESSAGE environment variable not set or empty");
    }
    
    let client = Client::new();
    match client.get(PING_PONG_URL).send().await {
        Ok(response) => {
            match response.text().await {
                Ok(pings) => format!("{}{}{}", output, timestamp_str, format!("\n{}", pings.trim())),
                Err(_) => format!("{}{}", output, timestamp_str),
            }
        }
        Err(_) => format!("{}{}", output, timestamp_str),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let application_type = env::var("APPLICATION_TYPE").unwrap_or_else(|_| "normal".to_string());
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
        let app = if application_type == "reader" {
            info!("Starting in reader mode");
            Router::new().route("/", get(move || async move {
                handle_root(log_path).await
            }))
        } else {
            info!("Starting in normal mode");
            Router::new().route("/", get(handle_normal))
        };

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
