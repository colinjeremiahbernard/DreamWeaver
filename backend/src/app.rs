use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::routes::{
    health::health,
    agents, // <-- CHANGED: Import the whole agents module
    workflow::workflow,
    creative_director::routes as creative_director_routes,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { "DreamWeaver Backend 🚀" }))
        .route("/health", get(health))
        .route("/agents", get(agents::agents))          // Works perfectly now!
        .route("/run_agents", post(agents::run_agents))  // Works perfectly now!
        .route("/workflow", post(workflow))
        .merge(creative_director_routes())
        .layer(CorsLayer::permissive())
}