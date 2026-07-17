use axum::{extract::Json, response::IntoResponse};
use serde::Deserialize;

use crate::creative_director::director::CreativeDirector;

#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub prompt: String,
}

pub async fn analyze(
    Json(request): Json<AnalyzeRequest>,
) -> impl IntoResponse {
    let director = CreativeDirector::new();

    Json(director.analyze(&request.prompt))
}