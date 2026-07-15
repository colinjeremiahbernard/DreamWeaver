use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{
    agents::available_agents,
    services::agent_executor::AgentExecutor,
};

#[derive(Deserialize)]
pub struct WorkflowRequest {
    pub idea: String,
}

#[derive(Serialize)]
pub struct WorkflowResponse {
    pub outputs: Vec<String>,
}

pub async fn workflow(
    Json(request): Json<WorkflowRequest>,
) -> Json<WorkflowResponse> {

    let ai_service = crate::services::ai_service::AiService::new();

    let executor = AgentExecutor::new(ai_service);

    let mut outputs = Vec::new();

for agent in available_agents() {
    let output = executor.execute(&agent, &request.idea).await;
    outputs.push(output);
}

    Json(WorkflowResponse {
        outputs,
    })
}