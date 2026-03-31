#!/usr/bin/env python3

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any

ROOT_DIR = Path(__file__).resolve().parent.parent


def load_env() -> None:
    env_path = ROOT_DIR / ".env"
    if not env_path.exists():
        return

    for line in env_path.read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        os.environ.setdefault(key, value)


def compose(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [
            "docker",
            "compose",
            "--env-file",
            str(ROOT_DIR / ".env"),
            "-f",
            str(ROOT_DIR / "compose.yaml"),
            "-f",
            str(ROOT_DIR / "compose.override.yaml"),
            *args,
        ],
        check=True,
        text=True,
        capture_output=True,
    )


def parse_compose_ps(output: str) -> list[dict[str, Any]]:
    text = output.strip()
    if not text:
        return []
    if text.startswith("["):
        return json.loads(text)
    return [json.loads(line) for line in text.splitlines() if line.strip()]


def fetch(
    url: str,
    *,
    method: str = "GET",
    payload: dict[str, Any] | None = None,
    headers: dict[str, str] | None = None,
) -> tuple[int, bytes, dict[str, str]]:
    body = None
    request_headers = dict(headers or {})
    if payload is not None:
        body = json.dumps(payload).encode("utf-8")
        request_headers.setdefault("Content-Type", "application/json")
    request = urllib.request.Request(url, data=body, method=method, headers=request_headers)
    with urllib.request.urlopen(request, timeout=30) as response:
        return response.status, response.read(), dict(response.headers.items())


def extract_context(html: str) -> dict[str, Any]:
    match = re.search(r"window\.context = (\{.*?\})\s*window\.pageError", html, re.S)
    if not match:
        raise RuntimeError("window.context not found on sign-in page")
    return json.loads(match.group(1))


def main() -> int:
    load_env()

    sourcegraph_url = os.environ.get("SOURCEGRAPH_EXTERNAL_URL", "http://127.0.0.1:7080")
    mcp_url = f"http://{os.environ.get('MCP_BIND_ADDRESS', '127.0.0.1')}:{os.environ.get('MCP_PORT', '7081')}/mcp"
    health_url = f"http://{os.environ.get('MCP_BIND_ADDRESS', '127.0.0.1')}:{os.environ.get('MCP_PORT', '7081')}/healthz"

    ps = parse_compose_ps(compose("ps", "--format", "json").stdout)
    services = {entry["Service"]: entry for entry in ps}

    required = [
        "caddy",
        "sourcegraph-frontend-0",
        "sourcegraph-frontend-internal",
        "mcp-bridge",
        "gitserver-0",
        "zoekt-webserver-0",
    ]
    missing = [name for name in required if name not in services]
    if missing:
        raise RuntimeError(f"missing required services in compose ps: {missing}")

    unhealthy = []
    for name in required:
        state = services[name].get("State", "")
        health = services[name].get("Health", "")
        if state != "running":
            unhealthy.append(f"{name}: state={state}")
            continue
        if health and health not in {"healthy", "starting"}:
            unhealthy.append(f"{name}: health={health}")
    if unhealthy:
        raise RuntimeError("service readiness failed: " + ", ".join(unhealthy))

    status, _, headers = fetch(sourcegraph_url)
    if status not in {200, 302}:
        raise RuntimeError(f"unexpected HTTP status for {sourcegraph_url}: {status}")

    _, sign_in_html, _ = fetch(f"{sourcegraph_url}/sign-in")
    context = extract_context(sign_in_html.decode("utf-8", errors="replace"))

    _, mcp_health, _ = fetch(health_url)
    if json.loads(mcp_health.decode("utf-8")) != {"ok": True}:
        raise RuntimeError("MCP health check returned an unexpected payload")

    initialize = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "sourcegraph-smoke", "version": "0.1.0"},
        },
    }
    _, init_body, _ = fetch(mcp_url, method="POST", payload=initialize)
    init_result = json.loads(init_body.decode("utf-8"))
    if "result" not in init_result:
        raise RuntimeError(f"MCP initialize failed: {init_result}")

    tools_list = {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {},
    }
    _, tools_body, _ = fetch(mcp_url, method="POST", payload=tools_list)
    tools_result = json.loads(tools_body.decode("utf-8"))
    tool_names = [tool["name"] for tool in tools_result["result"]["tools"]]

    list_repos = {
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "list_repos",
            "arguments": {"query": "", "limit": 1},
        },
    }
    _, repos_body, _ = fetch(mcp_url, method="POST", payload=list_repos)
    repos_result = json.loads(repos_body.decode("utf-8"))
    if repos_result.get("result", {}).get("isError"):
        raise RuntimeError(f"MCP repo listing failed: {repos_result}")
    repo_listing = repos_result["result"]["structuredContent"]

    report = {
        "services": {
            name: {
                "state": services[name].get("State"),
                "health": services[name].get("Health"),
            }
            for name in required
        },
        "sourcegraph": {
            "url": sourcegraph_url,
            "status": status,
            "location": headers.get("Location"),
            "needsSiteInit": context.get("needsSiteInit"),
            "needsRepositoryConfiguration": context.get("needsRepositoryConfiguration"),
            "accessTokensAllow": context.get("accessTokensAllow"),
        },
        "mcp": {
            "url": mcp_url,
            "tools": tool_names,
            "listRepos": {
                "count": len(repo_listing.get("nodes", [])),
                "hasNextPage": repo_listing.get("pageInfo", {}).get("hasNextPage"),
            },
        },
    }
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (RuntimeError, subprocess.CalledProcessError, urllib.error.URLError) as exc:
        print(f"smoke failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
