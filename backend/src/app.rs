use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::routes::{
    health::health,
    agents, 
    streaming,
    workflow::workflow,
    creative_director::routes as creative_director_routes,
};
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router { 
    // 1. Build the stateful routes first and supply their state immediately
    let main_routes = Router::new()
        .route("/", get(|| async { "DreamWeaver Backend 🚀" }))
        .route("/health", get(health))
        // NOTE: Make sure `pub async fn agents(...)` exists in agents.rs, 
        // or remove this line if it was an old placeholder!
        .route("/agents", get(agents::agents))          
        .route("/run_agents", post(streaming::run_agents_stream))  
        .route("/workflow", post(workflow))
        .route("/history", get(agents::get_history))              
        // Fixed: Reverted from {id} back to :id because Axum requires colon syntax 
        // for path extraction (e.g. Path(id): Path<i64>).
        .route("/history/{id}", get(agents::get_history_item)) 
        .with_state(state); 

    // 2. Merge with the stateless creative director routes and apply CORS
    main_routes
        .merge(creative_director_routes()) 
        .layer(CorsLayer::permissive())
}