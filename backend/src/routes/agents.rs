use axum::Json;
use serde::Serialize;

use crate::services::orchestrator;

#[derive(Serialize)]
pub struct AgentResponse {
    name: String,
    role: String,
}
pub async fn agents() -> Json<Vec<AgentResponse>> {
    // If you deleted this function or renamed it, restore it like this:
    let agents = crate::agents::available_agents() // Note: Fully qualified path to avoid the unused warning
        .into_iter()
        .map(|agent| AgentResponse {
            name: agent.name,
            role: agent.role,
        })
        .collect();

    Json(agents)
}

pub async fn run_agents(
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {

    let project = payload["project_id"].to_string();

    // 1. Capture and destructure the tuple returned from the orchestrator
    let (story, visuals, research, critique) = orchestrator::run_workflow(project.clone()).await;

    // 2. Now these variables exist in this scope and can be used below!
    Json(serde_json::json!({
        "status": "completed",
        "project_id": project,
        "artifacts": {
            "story": story,
            "visuals": visuals,
            "research": research,
            "critique": critique
        }
    }))
}