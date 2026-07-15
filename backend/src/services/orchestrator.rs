use crate::agents::Agent;

pub struct OrchestratorService;

impl OrchestratorService {
    pub fn plan_workflow(&self, idea: &str) -> Vec<String> {
        vec![
            format!("Analyze idea: {}", idea),
            "Assign researcher agent".to_string(),
            "Assign writer agent".to_string(),
            "Assign designer agent".to_string(),
            "Generate final creative output".to_string(),
        ]
    }
}