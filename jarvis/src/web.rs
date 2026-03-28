use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::Json;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::agent::Agent;

// ── State ───────────────────────────────────────────────────────────────

pub type AppState = Arc<Mutex<Agent>>;

// ── Request / Response ──────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    pub reply: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ── Routes ──────────────────────────────────────────────────────────────

pub fn router(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/", get(index_handler))
        .route("/api/chat", post(chat_handler))
        .with_state(state)
}

async fn index_handler() -> impl IntoResponse {
    Html(INDEX_HTML)
}

async fn chat_handler(
    State(agent): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> impl IntoResponse {
    let message = req.message.trim().to_string();
    if message.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ChatResponse {
                reply: String::new(),
                error: Some("message is empty".into()),
            }),
        );
    }

    let mut agent = agent.lock().await;
    match agent.process(&message).await {
        Ok(reply) => (
            StatusCode::OK,
            Json(ChatResponse {
                reply,
                error: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ChatResponse {
                reply: String::new(),
                error: Some(e),
            }),
        ),
    }
}

/// Start the web server.
pub async fn serve(agent: Agent, port: u16) {
    let state: AppState = Arc::new(Mutex::new(agent));
    let app = router(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    eprintln!("jarvis web UI: http://localhost:{port}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind address");
    axum::serve(listener, app)
        .await
        .expect("server error");
}

// ── Embedded HTML ───────────────────────────────────────────────────────

const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>jarvis</title>
<style>
  *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

  :root {
    --bg: #0d1117;
    --surface: #161b22;
    --border: #30363d;
    --text: #e6edf3;
    --muted: #7d8590;
    --accent: #58a6ff;
    --user-bg: #1f2937;
    --assistant-bg: #0d1117;
    --font: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    --mono: "SF Mono", "Fira Code", "Fira Mono", Menlo, Consolas, monospace;
  }

  body {
    font-family: var(--font);
    background: var(--bg);
    color: var(--text);
    height: 100vh;
    display: flex;
    flex-direction: column;
  }

  header {
    padding: 12px 20px;
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    gap: 10px;
    flex-shrink: 0;
  }

  header h1 {
    font-size: 16px;
    font-weight: 600;
    color: var(--accent);
  }

  header span {
    font-size: 12px;
    color: var(--muted);
  }

  #messages {
    flex: 1;
    overflow-y: auto;
    padding: 16px 20px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .msg {
    max-width: 80%;
    padding: 10px 14px;
    border-radius: 8px;
    line-height: 1.5;
    font-size: 14px;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .msg code {
    font-family: var(--mono);
    font-size: 13px;
    background: rgba(110, 118, 129, 0.15);
    padding: 1px 5px;
    border-radius: 4px;
  }

  .msg pre {
    background: #000;
    padding: 10px;
    border-radius: 6px;
    overflow-x: auto;
    margin: 6px 0;
    font-family: var(--mono);
    font-size: 13px;
  }

  .msg.user {
    align-self: flex-end;
    background: var(--user-bg);
    border: 1px solid var(--border);
  }

  .msg.assistant {
    align-self: flex-start;
    background: var(--assistant-bg);
    border: 1px solid var(--border);
  }

  .msg.error {
    align-self: flex-start;
    background: #3d1418;
    border: 1px solid #6e3630;
    color: #f47067;
  }

  .msg.system {
    align-self: center;
    color: var(--muted);
    font-size: 12px;
    padding: 4px 12px;
  }

  #input-area {
    padding: 12px 20px;
    border-top: 1px solid var(--border);
    display: flex;
    gap: 8px;
    flex-shrink: 0;
  }

  #input-area textarea {
    flex: 1;
    background: var(--surface);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 10px 14px;
    font-family: var(--font);
    font-size: 14px;
    resize: none;
    min-height: 44px;
    max-height: 200px;
    outline: none;
  }

  #input-area textarea:focus {
    border-color: var(--accent);
  }

  #input-area button {
    background: var(--accent);
    color: #fff;
    border: none;
    border-radius: 8px;
    padding: 0 20px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.15s;
    white-space: nowrap;
  }

  #input-area button:hover { opacity: 0.85; }
  #input-area button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .typing {
    align-self: flex-start;
    color: var(--muted);
    font-size: 13px;
    padding: 6px 14px;
  }

  .typing::after {
    content: '...';
    animation: dots 1.4s steps(4, end) infinite;
  }

  @keyframes dots {
    0%, 20% { content: ''; }
    40% { content: '.'; }
    60% { content: '..'; }
    80%, 100% { content: '...'; }
  }
</style>
</head>
<body>
  <header>
    <h1>jarvis</h1>
    <span>minimal coding agent</span>
  </header>

  <div id="messages">
    <div class="msg system">Type a message to start chatting with the coding agent.</div>
  </div>

  <div id="input-area">
    <textarea id="input" placeholder="Ask jarvis something..." rows="1"></textarea>
    <button id="send" onclick="sendMessage()">Send</button>
  </div>

<script>
const messages = document.getElementById('messages');
const input = document.getElementById('input');
const sendBtn = document.getElementById('send');

// Auto-resize textarea
input.addEventListener('input', () => {
  input.style.height = 'auto';
  input.style.height = Math.min(input.scrollHeight, 200) + 'px';
});

// Send on Enter (Shift+Enter for newline)
input.addEventListener('keydown', (e) => {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault();
    sendMessage();
  }
});

function addMessage(text, cls) {
  const el = document.createElement('div');
  el.className = 'msg ' + cls;
  el.textContent = text;
  messages.appendChild(el);
  messages.scrollTop = messages.scrollHeight;
  return el;
}

function addTyping() {
  const el = document.createElement('div');
  el.className = 'typing';
  el.id = 'typing';
  el.textContent = 'Thinking';
  messages.appendChild(el);
  messages.scrollTop = messages.scrollHeight;
  return el;
}

function removeTyping() {
  const el = document.getElementById('typing');
  if (el) el.remove();
}

async function sendMessage() {
  const text = input.value.trim();
  if (!text) return;

  addMessage(text, 'user');
  input.value = '';
  input.style.height = 'auto';
  sendBtn.disabled = true;
  input.disabled = true;

  addTyping();

  try {
    const resp = await fetch('/api/chat', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message: text }),
    });

    removeTyping();

    const data = await resp.json();
    if (data.error) {
      addMessage('Error: ' + data.error, 'error');
    } else {
      addMessage(data.reply || '(empty response)', 'assistant');
    }
  } catch (err) {
    removeTyping();
    addMessage('Network error: ' + err.message, 'error');
  } finally {
    sendBtn.disabled = false;
    input.disabled = false;
    input.focus();
  }
}

input.focus();
</script>
</body>
</html>
"##;

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use crate::config::Config;

    fn test_router() -> axum::Router {
        let config = Config::default();
        let agent = Agent::new(&config);
        let state: AppState = Arc::new(Mutex::new(agent));
        router(state)
    }

    #[tokio::test]
    async fn index_returns_html() {
        let app = test_router();
        let req = Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = String::from_utf8(body.to_vec()).unwrap();
        assert!(html.contains("<title>jarvis</title>"));
        assert!(html.contains("id=\"messages\""));
    }

    #[tokio::test]
    async fn chat_rejects_empty_message() {
        let app = test_router();
        let req = Request::builder()
            .uri("/api/chat")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"message": ""}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let data: ChatResponse = serde_json::from_slice(&body).unwrap();
        assert!(data.error.is_some());
    }

    #[tokio::test]
    async fn chat_rejects_invalid_json() {
        let app = test_router();
        let req = Request::builder()
            .uri("/api/chat")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from("not json"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        // Axum returns 400 for malformed JSON bodies
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
