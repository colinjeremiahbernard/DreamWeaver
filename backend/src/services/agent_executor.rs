use crate::{
    agents::Agent,
    services::ai_service::AiService,
};

pub struct AgentExecutor {
    ai_service: AiService,
}

impl AgentExecutor {
    pub fn new(ai_service: AiService) -> Self {
        Self { ai_service }
    }

    // Updated: Added model_override parameter
    pub async fn execute(&self, agent: &Agent, input: &str, model_override: Option<String>) -> String {
        println!("Running agent: {}", agent.name);

        let system_prompt = match agent.name.as_str() {
            "researcher" => {
                "You are an in-universe terminal retrieving raw database evidence. \
                 Output ONLY the actual primary source documents, classified historical logs, decrypted transcripts, and concrete recorded telemetry found in the world. \
                 Do NOT write structural templates, outlines, category lists, or future implications. \
                 Write raw, concrete facts, specific dates, and real in-universe historical files."
            }
            "writer" => {
                "You are a novelist writing the active scene. Output ONLY the finished narrative. \
                 Write active, immediate character actions, sensory details, and real dialogue. \
                 Do NOT write summaries, outlines, introductions, or commentary. Start immediately with the first line of the story."
            }
            "designer" => {
                "You are the cinematographer and set designer. Output ONLY concrete aesthetic specifications: \
                 actual camera frame directions, lens sizes (e.g. 35mm), precise color hex codes, lighting setups, and tactile set materials. \
                 Do NOT write generalized guidelines, blueprints, or planning bullet points. Provide the direct production specs."
            }
            "critic" => {
                "You are a fierce redline editor. Provide concrete, direct, line-by-line critical corrections on the story and visuals. \
                 Point out specific plot holes, weak words, or lighting inconsistencies. Do NOT write a meta-summary or general guidelines."
            }
            _ => "You are a helpful AI assistant.",
        };

        let prompt = format!(
            "System Directive: {}\n\n\
             Task Input:\n{}\n\n\
             STRICT CONTEXT RULE: Do NOT write outlines, bulleted categories of 'what should be researched', metadata, or structural frameworks. \
             You must output the actual, concrete, finalized factual evidence or content immediately:",
            system_prompt,
            input
        );

        // Updated: Pass the model override down to the AI service
        let result = self.ai_service.generate(&prompt, model_override).await;

        println!("Completed agent: {}", agent.name);
        // Find where the variable `result` is left standing alone at the bottom of the function:
    match result {
        Ok(text) => text,
        Err(e) => format!("Agent generation engine error: {}", e)
    }
}
    }
