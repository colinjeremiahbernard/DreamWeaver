use axum::{
    routing::post,
    Router,
};

use crate::handlers::creative_director;

pub fn routes() -> Router {
    Router::new()
        .route("/creative-director/analyze", post(creative_director::analyze))
}