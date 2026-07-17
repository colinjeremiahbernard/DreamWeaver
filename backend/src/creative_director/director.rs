use crate::models::creative_brief::CreativeBrief;
use uuid::Uuid;

pub struct CreativeDirector;

impl CreativeDirector {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, prompt: &str) -> CreativeBrief {
        CreativeBrief {
            id: Uuid::new_v4().to_string(),
            original_prompt: prompt.to_string(),
            title: Self::generate_title(prompt),
            premise: prompt.to_string(),
            genre: Self::detect_genre(prompt),
            tone: Self::detect_tone(prompt),
            audience: "General".to_string(),
            objectives: Self::objectives(),
            questions: Self::generate_questions(prompt),
            deliverables: Self::deliverables(),
            required_agents: Self::select_agents(prompt),
        }
    }

    fn generate_title(prompt: &str) -> String {
        if prompt.to_lowercase().contains("pirate")
            && prompt.to_lowercase().contains("ai")
        {
            "The Reclamation".to_string()
        } else {
            "Untitled Project".to_string()
        }
    }

    fn detect_genre(prompt: &str) -> Vec<String> {
        let p = prompt.to_lowercase();
        let mut genres = Vec::new();

        if p.contains("pirate") {
            genres.push("Adventure".into());
        }

        if p.contains("ai") || p.contains("robot") {
            genres.push("Science Fiction".into());
        }

        if p.contains("mystery") || p.contains("ancient") {
            genres.push("Mystery".into());
        }

        genres
    }

    fn detect_tone(prompt: &str) -> Vec<String> {
        let p = prompt.to_lowercase();
        let mut tones = Vec::new();

        if p.contains("ancient") {
            tones.push("Epic".into());
        }

        if p.contains("ocean") {
            tones.push("Mysterious".into());
        }

        if tones.is_empty() {
            tones.push("Creative".into());
        }

        tones
    }

    fn objectives() -> Vec<String> {
        vec![
            "Develop compelling narrative".into(),
            "Create immersive world".into(),
            "Design memorable characters".into(),
        ]
    }

    fn generate_questions(prompt: &str) -> Vec<String> {
        let mut questions = vec![
            "Who is the main protagonist?".into(),
            "What is the central conflict?".into(),
        ];

        if prompt.to_lowercase().contains("ai") {
            questions.push("Who created the AI?".into());
            questions.push("Why was it hidden?".into());
        }

        questions
    }

    fn deliverables() -> Vec<String> {
        vec![
            "Story Outline".into(),
            "Characters".into(),
            "World Design".into(),
            "Visual Concepts".into(),
        ]
    }

    fn select_agents(prompt: &str) -> Vec<String> {
        let mut agents = vec![
            "StoryAgent".into(),
            "CharacterAgent".into(),
            "CriticAgent".into(),
        ];

        let p = prompt.to_lowercase();

        if p.contains("world")
            || p.contains("ocean")
            || p.contains("city")
        {
            agents.push("WorldAgent".into());
        }

        agents.push("VisualAgent".into());

        agents
    }
}