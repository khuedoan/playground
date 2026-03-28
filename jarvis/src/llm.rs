use std::io::{self, Write};

use futures_util::StreamExt;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use crate::config::ProviderConfig;
use crate::tools::ToolCall;

// ── Request / Response types (OpenAI-compatible) ────────────────────────

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolSpec>>,
    pub max_tokens: u32,
    pub temperature: f32,
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallMsg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallMsg {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize)]
pub struct ToolSpec {
    #[serde(rename = "type")]
    pub spec_type: String,
    pub function: FunctionSpec,
}

#[derive(Debug, Serialize)]
pub struct FunctionSpec {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

// ── Streaming chunk types ───────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct StreamChunk {
    pub choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
pub struct StreamChoice {
    pub delta: DeltaContent,
    #[allow(dead_code)]
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeltaContent {
    pub content: Option<String>,
    pub tool_calls: Option<Vec<DeltaToolCall>>,
}

#[derive(Debug, Deserialize)]
pub struct DeltaToolCall {
    pub index: Option<usize>,
    pub id: Option<String>,
    pub function: Option<DeltaFunction>,
}

#[derive(Debug, Deserialize)]
pub struct DeltaFunction {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

// ── Response types ──────────────────────────────────────────────────────

/// Accumulated result from a streaming (or non-streaming) response.
pub enum LlmResponse {
    /// Plain text reply.
    Text(String),
    /// The model wants to call tools.
    ToolCalls(Vec<ToolCall>),
}

// ── Client ──────────────────────────────────────────────────────────────

pub struct LlmClient {
    http: reqwest::Client,
    config: ProviderConfig,
    api_key: String,
}

impl LlmClient {
    pub fn new(config: ProviderConfig) -> Self {
        let api_key = std::env::var("OPENAI_API_KEY")
            .or_else(|_| std::env::var("LLM_API_KEY"))
            .unwrap_or_default();

        Self {
            http: reqwest::Client::new(),
            config,
            api_key,
        }
    }

    /// Build the tool specifications the model can call.
    pub fn tool_specs() -> Vec<ToolSpec> {
        vec![
            ToolSpec {
                spec_type: "function".into(),
                function: FunctionSpec {
                    name: "read_file".into(),
                    description: "Read the contents of a file".into(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "path": { "type": "string", "description": "Relative path to the file" }
                        },
                        "required": ["path"]
                    }),
                },
            },
            ToolSpec {
                spec_type: "function".into(),
                function: FunctionSpec {
                    name: "write_file".into(),
                    description: "Write content to a file (creates or overwrites)".into(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "path": { "type": "string", "description": "Relative path to the file" },
                            "content": { "type": "string", "description": "File content to write" }
                        },
                        "required": ["path", "content"]
                    }),
                },
            },
            ToolSpec {
                spec_type: "function".into(),
                function: FunctionSpec {
                    name: "shell".into(),
                    description: "Run a shell command and return stdout/stderr".into(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "command": { "type": "string", "description": "The shell command to run" }
                        },
                        "required": ["command"]
                    }),
                },
            },
            ToolSpec {
                spec_type: "function".into(),
                function: FunctionSpec {
                    name: "search".into(),
                    description: "Search for a pattern in files (like grep -rn)".into(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "pattern": { "type": "string", "description": "Regex pattern to search for" },
                            "path": { "type": "string", "description": "Directory or file to search in (default: .)" }
                        },
                        "required": ["pattern"]
                    }),
                },
            },
        ]
    }

    /// Send a chat request with streaming and return the accumulated response.
    pub async fn chat(&self, messages: &[Message]) -> Result<LlmResponse, String> {
        let request = ChatRequest {
            model: self.config.model.clone(),
            messages: messages.to_vec(),
            tools: Some(Self::tool_specs()),
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            stream: true,
        };

        let url = format!("{}/chat/completions", self.config.api_base);
        let resp = self
            .http
            .post(&url)
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("API error {status}: {body}"));
        }

        // Stream SSE chunks
        let mut text_buf = String::new();
        let mut tool_calls_buf: Vec<(String, String, String)> = Vec::new(); // (id, name, args)

        let mut stream = resp.bytes_stream();
        let mut leftover = String::new();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| format!("Stream error: {e}"))?;
            let raw = format!("{leftover}{}", String::from_utf8_lossy(&chunk));
            leftover.clear();

            for line in raw.lines() {
                let line = line.trim();
                if line.is_empty() || line == "data: [DONE]" {
                    continue;
                }
                if let Some(json_str) = line.strip_prefix("data: ") {
                    match serde_json::from_str::<StreamChunk>(json_str) {
                        Ok(sc) => {
                            for choice in &sc.choices {
                                // Accumulate text
                                if let Some(ref c) = choice.delta.content {
                                    print!("{c}");
                                    let _ = io::stdout().flush();
                                    text_buf.push_str(c);
                                }
                                // Accumulate tool calls
                                if let Some(ref tcs) = choice.delta.tool_calls {
                                    for tc in tcs {
                                        let idx = tc.index.unwrap_or(0);
                                        while tool_calls_buf.len() <= idx {
                                            tool_calls_buf
                                                .push((String::new(), String::new(), String::new()));
                                        }
                                        if let Some(ref id) = tc.id {
                                            tool_calls_buf[idx].0 = id.clone();
                                        }
                                        if let Some(ref f) = tc.function {
                                            if let Some(ref name) = f.name {
                                                tool_calls_buf[idx].1 = name.clone();
                                            }
                                            if let Some(ref args) = f.arguments {
                                                tool_calls_buf[idx].2.push_str(args);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            // Possibly an incomplete line; save for next iteration
                            leftover = line.to_string();
                        }
                    }
                }
            }
        }

        if !text_buf.is_empty() {
            println!();
        }

        if tool_calls_buf.is_empty() {
            Ok(LlmResponse::Text(text_buf))
        } else {
            let calls = tool_calls_buf
                .into_iter()
                .map(|(id, name, args)| ToolCall { id, name, arguments: args })
                .collect();
            Ok(LlmResponse::ToolCalls(calls))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_specs_are_well_formed() {
        let specs = LlmClient::tool_specs();
        assert!(specs.len() >= 4);
        let names: Vec<&str> = specs.iter().map(|s| s.function.name.as_str()).collect();
        assert!(names.contains(&"read_file"));
        assert!(names.contains(&"write_file"));
        assert!(names.contains(&"shell"));
        assert!(names.contains(&"search"));
    }

    #[test]
    fn message_serialization_roundtrip() {
        let msg = Message {
            role: "user".into(),
            content: Some("hello".into()),
            tool_calls: None,
            tool_call_id: None,
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.role, "user");
        assert_eq!(parsed.content.as_deref(), Some("hello"));
    }
}
