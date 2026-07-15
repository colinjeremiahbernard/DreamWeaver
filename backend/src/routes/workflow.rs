use axum::Json;
use serde::{Deserialize, Serialize};

use crate::services::orchestrator::OrchestratorService;

#[derive(Deserialize)]
pub struct WorkflowRequest {
    pub idea: String,
}

#[derive(Serialize)]
pub struct WorkflowResponse {
    pub steps: Vec<String>,
}

pub async fn workflow(
    Json(request): Json<WorkflowRequest>,
) -> Json<WorkflowResponse> {

    let service = OrchestratorService;

    Json(WorkflowResponse {
        steps: service.plan_workflow(&request.idea),
    })
}