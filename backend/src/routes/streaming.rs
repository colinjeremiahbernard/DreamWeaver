use axum::{
    extract::State,
    response::sse::{Event, Sse},
    Json,
};
use futures_util::stream::Stream;
use futures_util::StreamExt; // 👈 Re-added this vital import so .next() works
use std::{convert::Infallible, pin::Pin};
use crate::state::AppState; 
use crate::services::ai_service::AiService;

/// Handles the real-time token streaming pipeline via Server-Sent Events (SSE)
pub async fn run_agents_stream(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Sse<Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>> {

    let project = payload["project_id"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| payload["project_id"].to_string());

    let custom_model = payload["model"].as_str().map(|s| s.to_string());

    let stream = async_stream::try_stream! {
        let ai = AiService::new();
        let mut final_research = String::new();
        let mut final_story = String::new();
        let mut final_visuals = String::new();
        let mut final_critique = String::new();

        macro_rules! stream_agent {
            ($agent_id:expr, $prompt:expr, $accumulator:expr) => {
                let current_agent = $agent_id.to_string();
                yield Event::default().event("agent_start").data(current_agent.clone());
                
                let mut token_stream = ai.generate_stream(&$prompt, custom_model.clone()).await;
                while let Some(token) = token_stream.next().await {
                    if !token.is_empty() {
                        $accumulator.push_str(&token);
                        yield Event::default().event("token").data(format!("{}:{}", current_agent, token));
                    }
                }
            };
        }

        // --- Execute Multi-Agent Chain ---
        let prompt_1 = format!("Generate a detailed world-building dossier for this premise: '{}'", project);
        stream_agent!("research", prompt_1, final_research);

        let prompt_2 = format!("Using this research lore:\n{}\n\nDraft the complete narrative scene for: '{}'", final_research, project);
        stream_agent!("story", prompt_2, final_story);

        let prompt_3 = format!("Based on this narrative scene:\n{}\n\nWrite visual art-direction production specs.", final_story);
        stream_agent!("visuals", prompt_3, final_visuals);

        let prompt_4 = format!("Analyze the text and visuals for consistency:\n\n[Text]\n{}\n\n[Visuals]\n{}", final_story, final_visuals);
        stream_agent!("critique", prompt_4, final_critique);

        // --- Save Completed Compilation to Database ---
        let db_write = sqlx::query(
            "INSERT INTO workflows (project_id, research, story, visuals, critique) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&project)
        .bind(&final_research)
        .bind(&final_story)
        .bind(&final_visuals)
        .bind(&final_critique)
        .execute(&state.db)
        .await;

        if let Err(e) = db_write {
            eprintln!("⚠️ Database stream log failed: {}", e);
        }

        yield Event::default().event("completed").data("done");
    };

    Sse::new(Box::pin(stream))
}