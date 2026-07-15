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

        let prompt = format!(
            "You are the {} agent. Your role is: {}. Task: {}",
            agent.name,
            agent.role,
            input
        );

        let result = self.ai_service.generate(&prompt).await;

        println!("Completed agent: {}", agent.name);

        result
    }
}