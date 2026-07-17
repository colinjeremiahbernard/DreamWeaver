use std::time::Duration;
use tokio::time::timeout;

use crate::{
    agents::Agent,
    services::{
        ai_service::AiService,
        agent_executor::AgentExecutor,
    },
};

// Set a safe time limit for an individual agent run (180 seconds to absorb model cold starts)
const STEP_TIMEOUT: Duration = Duration::from_secs(180);

// Updated: Accept model parameter from routing layer
pub async fn run_workflow(project: String, model: Option<String>) -> (String, String, String, String) {
    println!("--- Starting Sequenced Agent Workflow for: {} ---", project);

    let ai_service = AiService::new();
    let executor = AgentExecutor::new(ai_service);

    // --- 1. RESEARCHER AGENT ---
    let researcher = Agent {
        name: "researcher".to_string(),
        role: "Collects and analyzes information".to_string(),
        system_prompt: "".to_string(), 
    };
    let research_prompt = format!(
        "Generate a detailed world-building dossier, underlying rules of the universe, and technical context for this premise: '{}'",
        project
    );
    
    println!("🔍 Running Researcher Agent...");
    let research = match timeout(STEP_TIMEOUT, executor.execute(&researcher, &research_prompt, model.clone())).await {
        Ok(result) => result,
        Err(_) => {
            eprintln!("⚠️ Researcher timed out! Continuing with fallback state.");
            "⚠️ [Researcher Timeout] The database query took too long to resolve. Local LLM may be overloaded."
                .to_string()
        }
    };

    // --- 2. WRITER AGENT ---
    let writer = Agent {
        name: "writer".to_string(),
        role: "Creates written content".to_string(),
        system_prompt: "".to_string(),
    };
    let writer_prompt = format!(
        "Using this specific research lore:\n{}\n\nDraft the actual, complete narrative scene for this premise: '{}'",
        research, project
    );
    
    println!("✍️ Running Writer Agent...");
    let story = match timeout(STEP_TIMEOUT, executor.execute(&writer, &writer_prompt, model.clone())).await {
        Ok(result) => result,
        Err(_) => {
            eprintln!("⚠️ Writer timed out! Continuing with fallback state.");
            "⚠️ [Screenwriter Timeout] The story generation timed out. Please try running the workflow again."
                .to_string()
        }
    };

    // --- 3. DESIGNER AGENT ---
    let designer = Agent {
        name: "designer".to_string(),
        role: "Creates visual concepts".to_string(),
        system_prompt: "".to_string(),
    };
    let designer_prompt = format!(
        "Based on this exact narrative scene:\n{}\n\nWrite the comprehensive production art-direction notes, specifying camera frames, shadows, and environment design details.",
        story
    );
    
    println!("🎨 Running Designer Agent...");
    let visuals = match timeout(STEP_TIMEOUT, executor.execute(&designer, &designer_prompt, model.clone())).await {
        Ok(result) => result,
        Err(_) => {
            eprintln!("⚠️ Designer timed out! Continuing with fallback state.");
            "⚠️ [Visual Designer Timeout] Art direction generation timed out."
                .to_string()
        }
    };

    // --- 4. CRITIC AGENT ---
    let critic = Agent {
        name: "critic".to_string(),
        role: "Provides editorial review and feedback".to_string(),
        system_prompt: "".to_string(),
    };
    let critic_prompt = format!(
        "Analyze the following package for weaknesses. Write a strict editorial review critique.\n\n[Scene Text]\n{}\n\n[Visual Layout]\n{}",
        story, visuals
    );
    
    println!("⚖️ Running Critic Agent...");
    let review = match timeout(STEP_TIMEOUT, executor.execute(&critic, &critic_prompt, model.clone())).await {
        Ok(result) => result,
        Err(_) => {
            eprintln!("⚠️ Critic timed out! Continuing with fallback state.");
            "⚠️ [Creative Critic Timeout] Critic analysis timed out."
                .to_string()
        }
    };

    println!("--- Workflow Completed ---");

    (story, visuals, research, review)
}