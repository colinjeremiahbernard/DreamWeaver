use axum::{
    routing::get,
    Json,
    Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

#[tokio::main]
async fn main() {
    println!("🚀 DreamWeaver backend starting...");

    let app = Router::new()
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind server");

    println!("🌐 Server running at http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "dreamweaver-backend".to_string(),
        version: "0.1.0".to_string(),
    })
}