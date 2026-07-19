use serde::{Deserialize, Serialize};
use futures_util::Stream;
use std::pin::Pin;

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

#[derive(Deserialize)]
struct OllamaStreamChunk {
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

    /// Keep the classic batch generation intact so older engine components compile cleanly
    pub async fn generate(&self, prompt: &str, model_override: Option<String>) -> Result<String, String> {
        let client = reqwest::Client::new();
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
            .await
            .map_err(|e| e.to_string())?;

        let parsed = response
            .json::<OllamaResponse>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(parsed.response)
    }

    /// Real-time streaming generation method with string line buffering
    pub async fn generate_stream(
        &self, 
        prompt: &str, 
        model_override: Option<String>
    ) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        let client = reqwest::Client::new();
        let active_model = model_override.unwrap_or_else(|| self.model.clone());

        let request = OllamaRequest {
            model: active_model,
            prompt: prompt.to_string(),
            stream: true,
        };

        let response_stream = client
            .post(format!("{}/api/generate", self.url))
            .json(&request)
            .send()
            .await;

        match response_stream {
            Ok(res) => {
                let byte_stream = res.bytes_stream();
                // A buffer to hold onto partial JSON lines across network chunks
                let mut line_buffer = String::new();
                
                let token_stream = futures_util::StreamExt::map(byte_stream, move |item| {
                    match item {
                        Ok(bytes) => {
                            let raw_str = String::from_utf8_lossy(&bytes);
                            line_buffer.push_str(&raw_str);
                            
                            let mut combined = String::new();
                            
                            // Process only complete lines from the buffer
                            while let Some(newline_index) = line_buffer.find('\n') {
                                let complete_line = line_buffer[..newline_index].to_string();
                                line_buffer.drain(..=newline_index); // Remove processed line
                                
                                // Fixed: Removed the accidental double "if" keyword here
                                if let Ok(chunk) = serde_json::from_str::<OllamaStreamChunk>(&complete_line) {
                                    combined.push_str(&chunk.response);
                                }
                            }
                            combined
                        }
                        Err(_) => String::new(),
                    }
                });
                
                Box::pin(token_stream)
            }
            Err(e) => {
                Box::pin(futures_util::stream::once(async move {
                    format!("Ollama stream connection error: {}", e)
                }))
            }
        }
    }
}  