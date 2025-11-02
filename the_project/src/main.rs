use axum::{Router, response::Html, routing::get};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpListener;
use tracing::{error, info};

const IMAGE_PATH: &str = "/opt/project/pic.jpg";
const META_FILE_PATH: &str = "/opt/project/meta.json";
const PICSUM_URL: &str = "https://picsum.photos/1200";
const CACHE_TIME: u64 = 10 * 60; // 10 minutes in seconds

#[derive(Serialize, Deserialize, Debug)]
struct ImageMeta {
    last_fetched: u64,
}

fn read_meta() -> Option<ImageMeta> {
    match fs::read_to_string(META_FILE_PATH) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(meta) => Some(meta),
            Err(e) => {
                error!("Failed to parse meta.json: {}", e);
                None
            }
        },
        Err(e) => {
            error!("Failed to read meta.json: {}", e);
            None
        }
    }
}

fn write_meta(meta: &ImageMeta) {
    match serde_json::to_string(meta) {
        Ok(content) => {
            if let Err(e) = fs::write(META_FILE_PATH, content) {
                error!("Failed to write meta.json: {}", e);
            }
        }
        Err(e) => {
            error!("Failed to serialize meta.json: {}", e);
        }
    }
}

async fn fetch_image() {
    let mut meta = read_meta().unwrap_or(ImageMeta { last_fetched: 0 });
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let cache_expired = meta.last_fetched + CACHE_TIME < current_time;

    if cache_expired {
        let response = match reqwest::get(PICSUM_URL).await {
            Ok(resp) => resp,
            Err(e) => {
                error!("Failed to fetch image: {}", e);
                return;
            }
        };

        let bytes = match response.bytes().await {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to read image bytes: {}", e);
                return;
            }
        };

        if let Err(e) = fs::write(IMAGE_PATH, &bytes) {
            error!("Failed to write image to disk: {}", e);
            return;
        }

        meta.last_fetched = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        write_meta(&meta);
    } else {
        info!("Using cached image.");
        return;
    }
}

async fn home() -> Html<&'static str> {
    Html(
        r#"<h1>The Project App</h1>
        <img src="/image" alt="Random Image" width="600"/><br/>
        DevOps with Kubernetes 2025
        "#,
    )
}

async fn image_handler() -> Result<axum::response::Response, axum::http::StatusCode> {
    fetch_image().await;

    match fs::read(IMAGE_PATH) {
        Ok(image_data) => {
            let response = axum::response::Response::builder()
                .header("Content-Type", "image/jpeg")
                .body(axum::body::Body::from(image_data))
                .unwrap();
            Ok(response)
        }
        Err(e) => {
            error!("Failed to read image from disk: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let app = Router::new()
        .route("/", get(home))
        .route("/image", get(image_handler));

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
