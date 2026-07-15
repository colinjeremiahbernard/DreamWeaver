pub struct Agent {
    pub name: String,
    pub role: String,
}

pub fn available_agents() -> Vec<Agent> {
    vec![
        Agent {
            name: "writer".to_string(),
            role: "Creates written content".to_string(),
        },
        Agent {
            name: "designer".to_string(),
            role: "Creates visual concepts".to_string(),
        },
        Agent {
            name: "researcher".to_string(),
            role: "Collects and analyzes information".to_string(),
        },
        Agent {
            name: "orchestrator".to_string(),
            role: "Coordinates creative workflows".to_string(),
        },
    ]
}