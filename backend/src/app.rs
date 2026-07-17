use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::routes::{
    health::health,
    agents, 
    workflow::workflow,
    creative_director::routes as creative_director_routes,
};
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    // 1. Build the stateful routes first and supply their state immediately
    let main_routes = Router::new()
        .route("/", get(|| async { "DreamWeaver Backend 🚀" }))
        .route("/health", get(health))
        .route("/agents", get(agents::agents))          
        .route("/run_agents", post(agents::run_agents))  
        .route("/workflow", post(workflow))
        .route("/history", get(agents::get_history))              
        .route("/history/{id}", get(agents::get_history_by_id))    // Fixed: Changed :id to {id}
        .with_state(state); 

    // 2. Merge with the stateless creative director routes and apply CORS
    main_routes
        .merge(creative_director_routes()) 
        .layer(CorsLayer::permissive())
}