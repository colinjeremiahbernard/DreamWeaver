use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::{
  health::health,
  agents::agents,
  workflow::workflow,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { "DreamWeaver Backend 🚀" }))
        .route("/health", get(health))
        .route("/agents", get(agents))
        .route("/workflow", post(workflow))
}