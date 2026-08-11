#!/usr/bin/env python3
"""Deterministic OpenAI-compatible chat server for the Workbench E2E test."""

from __future__ import annotations

import argparse
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from typing import Any, Iterable


MODEL = "workbench-e2e-mock"
TOOL_CALL_ID = "call_workbench_demo"
TOOL_COMMAND = (
    "date -u +%FT%TZ > /workspace/workbench-demo.txt "
    "&& test -s /workspace/workbench-demo.txt"
)
FINAL_REPLY = "Created and verified workbench-demo.txt"


def _chunk(delta: dict[str, Any], finish_reason: str | None = None) -> dict[str, Any]:
    return {
        "id": "chatcmpl-workbench-e2e",
        "object": "chat.completion.chunk",
        "created": 0,
        "model": MODEL,
        "choices": [
            {
                "index": 0,
                "delta": delta,
                "finish_reason": finish_reason,
            }
        ],
    }


def completion_chunks(request: dict[str, Any]) -> Iterable[dict[str, Any]]:
    messages = request.get("messages")
    if not isinstance(messages, list):
        raise ValueError("messages must be an array")

    has_tool_result = any(
        isinstance(message, dict) and message.get("role") == "tool"
        for message in messages
    )
    if has_tool_result:
        yield _chunk({"role": "assistant", "content": FINAL_REPLY})
        yield _chunk({}, "stop")
    else:
        tools = request.get("tools", [])
        tool_names = {
            tool.get("function", {}).get("name")
            for tool in tools
            if isinstance(tool, dict) and isinstance(tool.get("function"), dict)
        }
        if "bash" not in tool_names:
            raise ValueError("Pi did not offer the bash tool")
        yield _chunk(
            {
                "role": "assistant",
                "tool_calls": [
                    {
                        "index": 0,
                        "id": TOOL_CALL_ID,
                        "type": "function",
                        "function": {
                            "name": "bash",
                            "arguments": json.dumps(
                                {"command": TOOL_COMMAND}, separators=(",", ":")
                            ),
                        },
                    }
                ],
            }
        )
        yield _chunk({}, "tool_calls")

    yield {
        "id": "chatcmpl-workbench-e2e",
        "object": "chat.completion.chunk",
        "created": 0,
        "model": MODEL,
        "choices": [],
        "usage": {
            "prompt_tokens": 1,
            "completion_tokens": 1,
            "total_tokens": 2,
        },
    }


class Handler(BaseHTTPRequestHandler):
    server_version = "workbench-mock-llm/1"

    def log_message(self, message: str, *args: object) -> None:
        print(f"{self.address_string()} - {message % args}", flush=True)

    def _json(self, status: int, body: dict[str, Any]) -> None:
        encoded = json.dumps(body, separators=(",", ":")).encode()
        self.send_response(status)
        self.send_header("content-type", "application/json")
        self.send_header("content-length", str(len(encoded)))
        self.end_headers()
        self.wfile.write(encoded)

    def do_GET(self) -> None:  # noqa: N802 - BaseHTTPRequestHandler API
        if self.path in {"/health", "/v1/health"}:
            self._json(200, {"status": "ok", "model": MODEL})
        elif self.path == "/v1/models":
            self._json(
                200,
                {"object": "list", "data": [{"id": MODEL, "object": "model"}]},
            )
        else:
            self._json(404, {"error": "not found"})

    def do_POST(self) -> None:  # noqa: N802 - BaseHTTPRequestHandler API
        if self.path not in {"/chat/completions", "/v1/chat/completions"}:
            self._json(404, {"error": "not found"})
            return

        try:
            content_length = int(self.headers.get("content-length", "0"))
            if not 0 < content_length <= 1_048_576:
                raise ValueError("invalid content length")
            request = json.loads(self.rfile.read(content_length))
            if not isinstance(request, dict):
                raise ValueError("request must be an object")
            chunks = list(completion_chunks(request))
        except (ValueError, json.JSONDecodeError) as error:
            self._json(400, {"error": str(error)})
            return

        self.send_response(200)
        self.send_header("content-type", "text/event-stream")
        self.send_header("cache-control", "no-cache")
        self.send_header("connection", "close")
        self.end_headers()
        for chunk in chunks:
            payload = json.dumps(chunk, separators=(",", ":"))
            self.wfile.write(f"data: {payload}\n\n".encode())
        self.wfile.write(b"data: [DONE]\n\n")
        self.wfile.flush()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", type=int, default=8080)
    args = parser.parse_args()
    server = ThreadingHTTPServer((args.host, args.port), Handler)
    print(f"mock LLM listening on http://{args.host}:{args.port}", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
