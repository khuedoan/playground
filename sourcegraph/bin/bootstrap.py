#!/usr/bin/env python3

from __future__ import annotations

import os
import sys
from pathlib import Path

ROOT_DIR = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT_DIR / "mcp-bridge"))

from sourcegraph_client import SourcegraphClient, SourcegraphError  # noqa: E402


def main() -> int:
    url = os.environ.get("SOURCEGRAPH_EXTERNAL_URL", "http://127.0.0.1:7080")
    client = SourcegraphClient(
        base_url=url,
        access_token=os.environ.get("SOURCEGRAPH_ACCESS_TOKEN") or None,
        username=os.environ.get("SOURCEGRAPH_ADMIN_USERNAME") or None,
        email=os.environ.get("SOURCEGRAPH_ADMIN_EMAIL") or None,
        password=os.environ.get("SOURCEGRAPH_ADMIN_PASSWORD") or None,
    )

    client.wait_until_ready()
    user = client.ensure_authenticated()
    print(f"Authenticated as {user['username']} on {url}")
    print(
        "Bootstrap verified Sourcegraph auth. Code host automation is applied in a"
        " later step once the live GraphQL schema has been validated."
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except SourcegraphError as exc:
        print(f"bootstrap failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
