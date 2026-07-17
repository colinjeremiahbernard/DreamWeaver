mod app;
mod routes;
mod handlers;
mod services;
mod models;
mod agents;
mod prompts;
mod config;
mod creative_director;
mod state;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    println!("🚀 DreamWeaver backend starting...");

    // 1. Establish local SQLite database connection pool
    let db_options = SqliteConnectOptions::from_str("sqlite://dreamweaver.db")
        .expect("Invalid database connection string")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(db_options)
        .await
        .expect("Failed to connect to SQLite");

    // 2. Automatically create the table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS workflows (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id TEXT NOT NULL,
            research TEXT NOT NULL,
            story TEXT NOT NULL,
            visuals TEXT NOT NULL,
            critique TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );"
    )
    .execute(&pool)
    .await
    .expect("Failed to initialize database tables");

    // 3. Wrap inside our shared State container
    let shared_state = AppState { db: pool };

    // 4. Pass the database state to your existing router generator
    let app = app::create_router(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind server");

    println!("🌐 Server running at http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}