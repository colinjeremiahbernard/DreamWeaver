use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub struct AiService {
    pub model: String,
    pub url: String,
}

impl AiService {
    pub fn new() -> Self {
        Self {
            model: std::env::var("OLLAMA_MODEL")
                .unwrap_or_else(|_| "qwen2.5:7b".to_string()),

            url: std::env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:11434".to_string()),
        }
    }

    // Updated: Accept an optional model string override from the request
    pub async fn generate(&self, prompt: &str, model_override: Option<String>) -> String {
        let client = reqwest::Client::new();

        // Use the override if present, otherwise fall back to the env/default model
        let active_model = model_override.unwrap_or_else(|| self.model.clone());

        let request = OllamaRequest {
            model: active_model,
            prompt: prompt.to_string(),
            stream: false,
        };

        let response = client
            .post(format!("{}/api/generate", self.url))
            .json(&request)
            .send()
            .await;

        match response {
            Ok(res) => {
                match res.json::<OllamaResponse>().await {
                    Ok(data) => data.response,
                    Err(e) => format!("Ollama response error: {}", e),
                }
            }
            Err(e) => format!("Ollama connection error: {}", e),
        }
    }
}