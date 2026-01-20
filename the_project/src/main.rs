use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Response},
    routing::{get, post},
    Json, Router,
};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpListener;
use tracing::{error, info};
use tokio_postgres::NoTls;

const IMAGE_PATH: &str = "/opt/project/pic.jpg";
const META_FILE_PATH: &str = "/opt/project/meta.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ImageMeta {
    last_fetched: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    text: String,
}

type DbClient = tokio_postgres::Client;

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
    let picsum_url = env::var("PICSUM_URL").unwrap_or_else(|_| "https://picsum.photos/1200".to_string());
    let cache_time = env::var("IMAGE_CACHE_TIME")
        .unwrap_or_else(|_| "600".to_string())
        .parse::<u64>()
        .unwrap_or(600);
    
    let mut meta = read_meta().unwrap_or(ImageMeta { last_fetched: 0 });
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let cache_expired = meta.last_fetched + cache_time < current_time;

    if cache_expired {
        let response = match reqwest::get(&picsum_url).await {
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
    }
}

async fn home() -> Html<String> {
    let backend_url = env::var("BACKEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let todos_url = format!("{}/todos", backend_url);
    
    let mut todos_html = String::new();
    match reqwest::get(&todos_url).await {
        Ok(resp) => {
            match resp.json::<Vec<String>>().await {
                Ok(todos) => {
                    for todo in todos {
                        todos_html.push_str(&format!("<li>{}</li>", todo));
                    }
                }
                Err(e) => {
                    error!("Failed to parse todos: {}", e);
                    todos_html.push_str("<li>Error loading todos</li>");
                }
            }
        }
        Err(e) => {
            error!("Failed to fetch todos from {}: {}", todos_url, e);
            todos_html.push_str("<li>Backend unreachable</li>");
        }
    }

    Html(format!(
        r#"
        <h1>The Project App</h1>
        <img src="/image" alt="Random Image" width="600"/><br/>

        <section>
            <label for="todo-input">New todo (max 140 chars):</label><br/>
            <input id="todo-input" type="text" maxlength="140"/>
            <button id="send-btn" type="button" onclick="sendTodo()">Create todo</button>

            <ul id="todos">
                {}
            </ul>
        </section>

        <script>
            async function sendTodo() {{
                const input = document.getElementById('todo-input');
                const text = input.value;
                if (!text) return;
                
                await fetch('/todos', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify({{ text }})
                }});
                input.value = '';
                window.location.reload();
            }}
        </script>

        DevOps with Kubernetes 2025
        "#,
        todos_html
    ))
}

async fn image_handler() -> Result<Response, StatusCode> {
    fetch_image().await;

    match fs::read(IMAGE_PATH) {
        Ok(image_data) => {
            let response = Response::builder()
                .header("Content-Type", "image/jpeg")
                .body(axum::body::Body::from(image_data))
                .unwrap();
            Ok(response)
        }
        Err(e) => {
            error!("Failed to read image from disk: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Backend handlers
async fn get_todos(State(client): State<Arc<DbClient>>) -> Json<Vec<String>> {
    match client.query("SELECT text FROM todos ORDER BY id ASC", &[]).await {
        Ok(rows) => {
            let todos: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
            Json(todos)
        }
        Err(e) => {
            error!("Failed to fetch todos: {}", e);
            Json(vec![])
        }
    }
}

async fn post_todo(
    State(client): State<Arc<DbClient>>,
    Json(payload): Json<Todo>,
) -> StatusCode {
    if payload.text.len() > 140 {
        return StatusCode::BAD_REQUEST;
    }
    
    match client
        .execute("INSERT INTO todos (text) VALUES ($1)", &[&payload.text])
        .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            error!("Failed to insert todo: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_type = env::var("APPLICATION_TYPE").unwrap_or_else(|_| "app".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    if app_type == "backend" {
        info!("Starting in backend mode on port {}", port);
        
        // Get database URL from env or build from components
        let database_url = match env::var("DATABASE_URL") {
            Ok(u) => u,
            Err(_) => {
                let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
                let pass = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
                let db = env::var("POSTGRES_DB").unwrap_or_else(|_| "todos".to_string());
                let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "postgres-todos-0.postgres-todos-headless".to_string());
                let db_port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
                format!("host={} user={} password={} dbname={} port={}", host, user, pass, db, db_port)
            }
        };
        
        // Connect to postgres with retries
        let mut attempt = 0u32;
        let (client, connection) = loop {
            match tokio_postgres::connect(&database_url, NoTls).await {
                Ok(c) => break c,
                Err(e) => {
                    attempt += 1;
                    if attempt >= 15 {
                        error!("Failed to connect to database after {} attempts: {}", attempt, e);
                        return;
                    }
                    error!("Postgres not ready yet (attempt {}): {}. retrying...", attempt, e);
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            }
        };
        
        // Spawn connection driver
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                error!("Postgres connection error: {}", e);
            }
        });
        
        // Initialize schema
        if let Err(e) = client
            .execute(
                "CREATE TABLE IF NOT EXISTS todos (id SERIAL PRIMARY KEY, text TEXT NOT NULL)",
                &[],
            )
            .await
        {
            error!("Failed to create todos table: {}", e);
        }
        
        // Insert initial todos if table is empty
        let row_count: i64 = client
            .query_one("SELECT COUNT(*) FROM todos", &[])
            .await
            .map(|row| row.get(0))
            .unwrap_or(0);
        
        if row_count == 0 {
            let default_todos = vec![
                "Buy groceries",
                "Build a robot who takes over the world",
                "Eat frozen yogurt",
            ];
            
            for todo in default_todos {
                if let Err(e) = client.execute("INSERT INTO todos (text) VALUES ($1)", &[&todo]).await {
                    error!("Failed to insert default todo: {}", e);
                }
            }
        }
        
        let client = Arc::new(client);

        let app = Router::new()
            .route("/todos", get(get_todos).post(post_todo))
            .with_state(client);

        let listener = match TcpListener::bind(format!("0.0.0.0:{}", port)).await {
            Ok(l) => l,
            Err(e) => {
                error!("Failed to bind to port {}: {}", port, e);
                return;
            }
        };
        axum::serve(listener, app).await.unwrap();
    } else {
        info!("Starting in app mode on port {}", port);
        let app = Router::new()
            .route("/", get(home))
            .route("/image", get(image_handler));

        let listener = match TcpListener::bind(format!("0.0.0.0:{}", port)).await {
            Ok(l) => l,
            Err(e) => {
                error!("Failed to bind to port {}: {}", port, e);
                return;
            }
        };
        axum::serve(listener, app).await.unwrap();
    }
}
