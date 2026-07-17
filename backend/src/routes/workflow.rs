use axum::Json;
use futures::future::join_all;
use serde::{Deserialize, Serialize};

use crate::{
    agents::available_agents,
    services::{
        agent_executor::AgentExecutor,
        ai_service::AiService,
    },
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

    let tasks = available_agents().into_iter().map(|agent| {
        let idea = request.idea.clone();

        async move {
            let ai_service = AiService::new();
            let executor = AgentExecutor::new(ai_service);

            executor.execute(&agent, &idea).await
        }
    });

    let outputs = join_all(tasks).await;

    Json(WorkflowResponse {
        outputs,
    })
}