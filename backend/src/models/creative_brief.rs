use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeBrief {
    pub id: String,
    pub original_prompt: String,
    pub title: String,
    pub premise: String,
    pub genre: Vec<String>,
    pub tone: Vec<String>,
    pub audience: String,
    pub objectives: Vec<String>,
    pub questions: Vec<String>,
    pub deliverables: Vec<String>,
    pub required_agents: Vec<String>,
}