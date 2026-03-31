#!/usr/bin/env python3

from __future__ import annotations

import json
import os
from http import HTTPStatus
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from typing import Any

from sourcegraph_client import SourcegraphClient, SourcegraphError


def json_dumps(data: Any) -> bytes:
    return json.dumps(data, indent=2, sort_keys=True).encode("utf-8")


CLIENT = SourcegraphClient(
    base_url=os.environ.get("SOURCEGRAPH_URL", "http://caddy"),
    access_token=os.environ.get("SOURCEGRAPH_ACCESS_TOKEN") or None,
    username=os.environ.get("SOURCEGRAPH_USERNAME") or None,
    email=os.environ.get("SOURCEGRAPH_EMAIL") or None,
    password=os.environ.get("SOURCEGRAPH_PASSWORD") or None,
)


class Handler(BaseHTTPRequestHandler):
    server_version = "sourcegraph-mcp/0.1"

    def do_GET(self) -> None:
        if self.path == "/healthz":
            self._send_json(HTTPStatus.OK, {"ok": True})
            return

        self._send_json(HTTPStatus.NOT_FOUND, {"error": "not found"})

    def do_POST(self) -> None:
        if self.path != "/mcp":
            self._send_json(HTTPStatus.NOT_FOUND, {"error": "not found"})
            return

        length = int(self.headers.get("Content-Length", "0"))
        payload = self.rfile.read(length) if length else b"{}"

        try:
            request = json.loads(payload)
        except json.JSONDecodeError as exc:
            self._send_json(
                HTTPStatus.BAD_REQUEST,
                {"error": f"invalid json: {exc}"},
            )
            return

        response = self._handle_rpc(request)
        self._send_json(HTTPStatus.OK, response)

    def _handle_rpc(self, request: dict[str, Any]) -> dict[str, Any]:
        request_id = request.get("id")
        method = request.get("method")
        params = request.get("params") or {}

        try:
            if method == "initialize":
                return self._result(
                    request_id,
                    {
                        "protocolVersion": "2024-11-05",
                        "serverInfo": {
                            "name": "sourcegraph-local-bridge",
                            "version": "0.1.0",
                        },
                        "capabilities": {
                            "tools": {},
                        },
                    },
                )

            if method == "notifications/initialized":
                return self._result(request_id, {})

            if method == "ping":
                return self._result(request_id, {})

            if method == "tools/list":
                return self._result(request_id, {"tools": self._tools()})

            if method == "tools/call":
                name = params.get("name")
                arguments = params.get("arguments") or {}
                return self._result(request_id, self._call_tool(name, arguments))

            return self._error(request_id, -32601, f"method not found: {method}")
        except SourcegraphError as exc:
            return self._result(
                request_id,
                {
                    "content": [{"type": "text", "text": f"Sourcegraph error: {exc}"}],
                    "isError": True,
                },
            )
        except Exception as exc:  # pragma: no cover
            return self._error(request_id, -32000, f"internal error: {exc}")

    def _tools(self) -> list[dict[str, Any]]:
        return [
            {
                "name": "list_repos",
                "description": "List repositories visible in Sourcegraph.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query": {"type": "string"},
                        "limit": {"type": "integer", "minimum": 1, "maximum": 100},
                    },
                },
            },
            {
                "name": "search_code",
                "description": "Search code with a Sourcegraph search query.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query": {"type": "string"},
                        "display": {"type": "integer", "minimum": 1, "maximum": 100},
                    },
                    "required": ["query"],
                },
            },
            {
                "name": "read_file",
                "description": "Read a file from a repository.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "repo": {"type": "string"},
                        "path": {"type": "string"},
                        "revision": {"type": "string"},
                    },
                    "required": ["repo", "path"],
                },
            },
            {
                "name": "list_directory",
                "description": "List files in a repository directory.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "repo": {"type": "string"},
                        "path": {"type": "string"},
                        "revision": {"type": "string"},
                    },
                    "required": ["repo"],
                },
            },
            {
                "name": "search_commits",
                "description": "Search commits with a Sourcegraph search query.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query": {"type": "string"},
                        "display": {"type": "integer", "minimum": 1, "maximum": 100},
                    },
                    "required": ["query"],
                },
            },
        ]

    def _call_tool(self, name: str, arguments: dict[str, Any]) -> dict[str, Any]:
        if name == "list_repos":
            result = CLIENT.list_repositories(
                query=arguments.get("query", ""),
                limit=int(arguments.get("limit", 20)),
            )
            return self._tool_result(result)

        if name == "search_code":
            result = CLIENT.search_code(
                query=arguments["query"],
                display=int(arguments.get("display", 20)),
            )
            return self._tool_result(result)

        if name == "read_file":
            result = CLIENT.read_file(
                repo=arguments["repo"],
                path=arguments["path"],
                revision=arguments.get("revision"),
            )
            return self._tool_result(result)

        if name == "list_directory":
            result = CLIENT.list_directory(
                repo=arguments["repo"],
                path=arguments.get("path", ""),
                revision=arguments.get("revision"),
            )
            return self._tool_result(result)

        if name == "search_commits":
            result = CLIENT.search_commits(
                query=arguments["query"],
                display=int(arguments.get("display", 20)),
            )
            return self._tool_result(result)

        raise SourcegraphError(f"unknown tool: {name}")

    def _tool_result(self, result: Any) -> dict[str, Any]:
        return {
            "content": [{"type": "text", "text": json.dumps(result, indent=2)}],
            "structuredContent": result,
            "isError": False,
        }

    def _result(self, request_id: Any, result: Any) -> dict[str, Any]:
        return {"jsonrpc": "2.0", "id": request_id, "result": result}

    def _error(self, request_id: Any, code: int, message: str) -> dict[str, Any]:
        return {
            "jsonrpc": "2.0",
            "id": request_id,
            "error": {"code": code, "message": message},
        }

    def _send_json(self, status: HTTPStatus, payload: Any) -> None:
        body = json_dumps(payload)
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, format: str, *args: Any) -> None:  # noqa: A003
        return


def main() -> int:
    port = int(os.environ.get("PORT", "7081"))
    server = ThreadingHTTPServer(("0.0.0.0", port), Handler)
    server.serve_forever()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
