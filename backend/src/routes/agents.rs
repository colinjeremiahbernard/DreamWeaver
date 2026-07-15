use axum::Json;
use serde::Serialize;

use crate::agents::available_agents;

#[derive(Serialize)]
pub struct AgentResponse {
    name: String,
    role: String,
}

pub async fn agents() -> Json<Vec<AgentResponse>> {
    let agents = available_agents()
        .into_iter()
        .map(|agent| AgentResponse {
            name: agent.name,
            role: agent.role,
        })
        .collect();

    Json(agents)
}