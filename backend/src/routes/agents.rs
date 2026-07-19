use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use crate::AppState;

/// Fetches all archived items for the history sidebar
pub async fn get_history(State(state): State<AppState>) -> impl IntoResponse {
    // Dynamic query bypasses the compile-time DATABASE_URL requirement
    let records = sqlx::query("SELECT id, project_id, created_at FROM workflows ORDER BY id DESC")
        .fetch_all(&state.db)
        .await;

    match records {
        Ok(rows) => {
            use sqlx::Row;
            let list: Vec<serde_json::Value> = rows.into_iter().map(|row| {
                serde_json::json!({
                    "id": row.get::<i64, _>("id"),
                    "project_id": row.get::<String, _>("project_id"),
                    "created_at": row.get::<String, _>("created_at")
                })
            }).collect();
            Json(list).into_response()
        }
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
pub async fn agents() -> &'static str {
    "Agents endpoint placeholder"
}

/// Fetches a specific full suite record details
pub async fn get_history_item(Path(id): Path<i64>, State(state): State<AppState>) -> impl IntoResponse {
    let record = sqlx::query("SELECT * FROM workflows WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await;

    match record {
        Ok(Some(row)) => {
            use sqlx::Row;
            Json(serde_json::json!({
                "id": row.get::<i64, _>("id"),
                "project_id": row.get::<String, _>("project_id"),
                "research": row.get::<String, _>("research"),
                "story": row.get::<String, _>("story"),
                "visuals": row.get::<String, _>("visuals"),
                "critique": row.get::<String, _>("critique"),
                "created_at": row.get::<String, _>("created_at")
            })).into_response()
        }
        Ok(None) => axum::http::StatusCode::NOT_FOUND.into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}