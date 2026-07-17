use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub name: String,
    pub role: String,
    pub system_prompt: String,
}

pub fn available_agents() -> Vec<Agent> {
    vec![
        Agent {
            name: "writer".to_string(),
            role: "Creates written content".to_string(),
            system_prompt: "You are a master creative writer. Draft rich, engaging, and atmospheric narrative content based on the provided project ideas and research.".to_string(),
        },
        Agent {
            name: "designer".to_string(),
            role: "Creates visual concepts".to_string(),
            system_prompt: "You are a visual designer and prompt engineer. Describe vivid visual concepts, art directions, and aesthetic cues suitable for image generation prompts based on the story theme.".to_string(),
        },
        Agent {
            name: "researcher".to_string(),
            role: "Collects and analyzes information".to_string(),
            system_prompt: "You are an analytical researcher. Gather historical context, technical details, thematic elements, and lore to enrich the project baseline before writing begins.".to_string(),
        },
        Agent {
            name: "critic".to_string(),
            role: "Provides editorial review and feedback".to_string(),
            system_prompt: "You are an exacting editor and literary critic. Evaluate the generated narrative and visual design for structural cohesion, pacing, emotional impact, and thematic consistency.".to_string(),
        },
    ]
}