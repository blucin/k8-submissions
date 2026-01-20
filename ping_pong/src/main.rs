use axum::extract::State;
use axum::{Router, routing::get};
use std::env;
// use std::io::Write;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};

use tokio_postgres::NoTls;

type DbClient = tokio_postgres::Client;

async fn pinger(State(client): State<Arc<DbClient>>) -> String {
    // increment counter in DB and return the new value
    let res = client
        .query_opt("UPDATE counters SET value = value + 1 WHERE name = $1 RETURNING value", &[&"pingpong"]) 
        .await;

    let n: i64 = match res {
        Ok(Some(row)) => row.get(0),
        Ok(None) => {
            // row missing, try to insert and set to 1
            match client.execute("INSERT INTO counters (name, value) VALUES ($1, 1) ON CONFLICT (name) DO UPDATE SET value = counters.value + 1", &[&"pingpong"]).await {
                Ok(_) => {
                    // fetch value
                    match client.query_one("SELECT value FROM counters WHERE name = $1", &[&"pingpong"]).await {
                        Ok(r) => r.get(0),
                        Err(e) => {
                            error!("Failed to read counter after insert: {}", e);
                            0
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to insert counter row: {}", e);
                    0
                }
            }
        }
        Err(e) => {
            error!("Failed to update counter: {}", e);
            0
        }
    };

    let data = format!("Ping / Pongs: {}", n);
    data
}

async fn pings(State(client): State<Arc<DbClient>>) -> String {
    match client.query_one("SELECT value FROM counters WHERE name = $1", &[&"pingpong"]).await {
        Ok(row) => {
            let v: i64 = row.get(0);
            format!("Pings / Pongs: {}", v)
        }
        Err(e) => {
            error!("Failed to read counter: {}", e);
            "Pings / Pongs: 0".to_string()
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    // DATABASE_URL can be provided via env, otherwise build from defaults and service DNS
    let database_url = match env::var("DATABASE_URL") {
        Ok(u) => u,
        Err(_) => {
            let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
            let pass = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
            let db = env::var("POSTGRES_DB").unwrap_or_else(|_| "pingpong".to_string());
            let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "postgres-0.postgres-headless".to_string());
            let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
            format!("host={} user={} password={} dbname={} port={}", host, user, pass, db, port)
        }
    };

    // connect to postgres with retries while the DB is starting
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

    // spawn connection driver
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Postgres connection error: {}", e);
        }
    });

    // initialize schema/table and ensure a row exists
    if let Err(e) = client
        .execute(
            "CREATE TABLE IF NOT EXISTS counters (id SERIAL PRIMARY KEY, name TEXT UNIQUE, value BIGINT)",
            &[],
        )
        .await
    {
        error!("Failed to create counters table: {}", e);
    }

    if let Err(e) = client
        .execute(
            "INSERT INTO counters (name, value) VALUES ($1, 0) ON CONFLICT (name) DO NOTHING",
            &[&"pingpong"],
        )
        .await
    {
        error!("Failed to insert initial counter row: {}", e);
    }

    let client = Arc::new(client);

    let app = Router::new()
        .route("/pingpong", get(pinger))
        .route("/pings", get(pings))
        .with_state(client);

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
