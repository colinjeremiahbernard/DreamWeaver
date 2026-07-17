use axum::{extract::{Path, State}, Json};
use serde::Serialize;
use crate::services::orchestrator;
use crate::state::AppState;

#[derive(Serialize)]
pub struct AgentResponse {
    name: String,
    role: String,
}

// Struct to represent a lightweight entry in the sidebar list
#[derive(Serialize, sqlx::FromRow)]
pub struct HistoryListItem {
    id: i64,
    project_id: String,
    created_at: String,
}

// Struct to represent the deep, singular project payload when loaded
#[derive(Serialize, sqlx::FromRow)]
pub struct FullHistoryRecord {
    id: i64,
    project_id: String,
    research: String,
    story: String,
    visuals: String,
    critique: String,
    created_at: String,
}

pub async fn agents() -> Json<Vec<AgentResponse>> {
    let agents = crate::agents::available_agents() 
        .into_iter()
        .map(|agent| AgentResponse {
            name: agent.name,
            role: agent.role,
        })
        .collect();

    Json(agents)
}

pub async fn run_agents(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {

    // Extract the project idea / text prompt safely
    let project = payload["project_id"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| payload["project_id"].to_string());

    // NEW: Safely extract the custom model string passed from the frontend UI dropdown settings
    let custom_model = payload["model"]
        .as_str()
        .map(|s| s.to_string());

    // 1. Updated: Pass both project prompt and the optional custom model override to the orchestrator
    let (story, visuals, research, critique) = orchestrator::run_workflow(project.clone(), custom_model).await;

    // 2. Persist the generated artifacts inside SQLite
    let insert_result = sqlx::query(
        "INSERT INTO workflows (project_id, research, story, visuals, critique) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&project)
    .bind(&research)
    .bind(&story)
    .bind(&visuals)
    .bind(&critique)
    .execute(&state.db)
    .await;

    match insert_result {
        Ok(_) => println!("💾 Saved creative assets for project '{}' to database.", project),
        Err(e) => eprintln!("⚠️ Database write failed: {}", e),
    }

    // 3. Return the exact response structure to your frontend
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

// 4. Fetch lightweight history metadata for the UI sidebar list
pub async fn get_history(
    State(state): State<AppState>,
) -> Result<Json<Vec<HistoryListItem>>, (axum::http::StatusCode, String)> {
    
    let items = sqlx::query_as::<_, HistoryListItem>(
        "SELECT id, project_id, datetime(created_at, 'localtime') as created_at \
         FROM workflows \
         ORDER BY id DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(items))
}

// 5. Fetch all raw research/story/design strings for a chosen record
pub async fn get_history_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<FullHistoryRecord>, (axum::http::StatusCode, String)> {

    let record = sqlx::query_as::<_, FullHistoryRecord>(
        "SELECT id, project_id, research, story, visuals, critique, datetime(created_at, 'localtime') as created_at \
         FROM workflows \
         WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (axum::http::StatusCode::NOT_FOUND, format!("Workflow record not found: {}", e)))?;

    Ok(Json(record))
}