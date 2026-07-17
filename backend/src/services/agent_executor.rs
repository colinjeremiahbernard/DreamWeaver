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

    pub async fn execute(&self, agent: &Agent, input: &str) -> String {
        println!("Running agent: {}", agent.name);

        let system_prompt = match agent.name.as_str() {
            "writer" => {
                "You are an award-winning screenwriter. Write compelling stories with strong characters, dialogue, pacing, and emotion."
            }

            "designer" => {
                "You are a senior concept artist. Describe environments, lighting, color palettes, costumes, architecture, and cinematic visuals."
            }

            "researcher" => {
                "You are a research specialist. Provide scientific accuracy, historical context, technical facts, and useful references."
            }

            "orchestrator" => {
                "You are a creative director. Combine ideas into one coherent production plan with clear next steps."
            }

            _ => "You are a helpful AI assistant.",
        };

        let prompt = format!(
            "{}\n\nUser Request:\n{}\n\nRespond only in your assigned role.",
            system_prompt,
            input
        );

        let result = self.ai_service.generate(&prompt).await;

        println!("Completed agent: {}", agent.name);

        result
    }
}