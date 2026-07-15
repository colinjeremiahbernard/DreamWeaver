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

    pub async fn generate(&self, prompt: &str) -> String {
        let client = reqwest::Client::new();

        let request = OllamaRequest {
            model: self.model.clone(),
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