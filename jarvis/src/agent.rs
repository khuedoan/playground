use crate::config::Config;
use crate::llm::{LlmClient, LlmResponse, Message, ToolCallMsg, FunctionCall};
use crate::sandbox::Sandbox;
use crate::tools;

/// The core agent loop: takes user input, calls the LLM, executes tools,
/// and iterates until the model produces a final text response.
pub struct Agent {
    client: LlmClient,
    sandbox: Sandbox,
    messages: Vec<Message>,
    max_rounds: usize,
}

impl Agent {
    pub fn new(config: &Config) -> Self {
        let system_msg = Message {
            role: "system".into(),
            content: Some(config.agent.system_prompt.clone()),
            tool_calls: None,
            tool_call_id: None,
        };

        Self {
            client: LlmClient::new(config.provider.clone()),
            sandbox: Sandbox::new(&config.sandbox),
            messages: vec![system_msg],
            max_rounds: config.agent.max_rounds,
        }
    }

    /// Process a single user message through the agent loop.
    pub async fn process(&mut self, user_input: &str) -> Result<String, String> {
        self.messages.push(Message {
            role: "user".into(),
            content: Some(user_input.into()),
            tool_calls: None,
            tool_call_id: None,
        });

        for round in 0..self.max_rounds {
            eprintln!("\n--- round {}/{} ---", round + 1, self.max_rounds);

            let response = self.client.chat(&self.messages).await?;

            match response {
                LlmResponse::Text(text) => {
                    self.messages.push(Message {
                        role: "assistant".into(),
                        content: Some(text.clone()),
                        tool_calls: None,
                        tool_call_id: None,
                    });
                    return Ok(text);
                }
                LlmResponse::ToolCalls(calls) => {
                    // Record the assistant message with tool calls
                    let tool_call_msgs: Vec<ToolCallMsg> = calls
                        .iter()
                        .map(|c| ToolCallMsg {
                            id: c.id.clone(),
                            call_type: "function".into(),
                            function: FunctionCall {
                                name: c.name.clone(),
                                arguments: c.arguments.clone(),
                            },
                        })
                        .collect();

                    self.messages.push(Message {
                        role: "assistant".into(),
                        content: None,
                        tool_calls: Some(tool_call_msgs),
                        tool_call_id: None,
                    });

                    // Execute each tool and record results
                    for call in &calls {
                        eprintln!("  → tool: {}({})", call.name, call.arguments);
                        let result = tools::execute(&self.sandbox, call);
                        eprintln!(
                            "  ← result: {}",
                            if result.len() > 200 {
                                format!("{}...", &result[..200])
                            } else {
                                result.clone()
                            }
                        );

                        self.messages.push(Message {
                            role: "tool".into(),
                            content: Some(result),
                            tool_calls: None,
                            tool_call_id: Some(call.id.clone()),
                        });
                    }
                }
            }
        }

        Err(format!(
            "agent exceeded maximum rounds ({})",
            self.max_rounds
        ))
    }

    /// Return a reference to the sandbox for external use.
    pub fn sandbox(&self) -> &Sandbox {
        &self.sandbox
    }
}
