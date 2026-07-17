
mod app;
mod routes;
mod handlers;
mod services;
mod models;
mod agents;
mod prompts;
mod config;
mod creative_director;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    println!("🚀 DreamWeaver backend starting...");

    let app = app::create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind server");

    println!("🌐 Server running at http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}