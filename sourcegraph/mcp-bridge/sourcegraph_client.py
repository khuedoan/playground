#!/usr/bin/env python3

from __future__ import annotations

import html.parser
import json
import re
import socket
import time
import urllib.error
import urllib.parse
import urllib.request
from http import cookiejar
from typing import Any


class SourcegraphError(RuntimeError):
    pass


class _FormParser(html.parser.HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.forms: list[dict[str, Any]] = []
        self._current: dict[str, Any] | None = None

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        attr_map = {key: value or "" for key, value in attrs}

        if tag == "form":
            self._current = {
                "action": attr_map.get("action", ""),
                "method": attr_map.get("method", "get").lower(),
                "inputs": [],
            }
            self.forms.append(self._current)
            return

        if tag == "input" and self._current is not None:
            self._current["inputs"].append(
                {
                    "name": attr_map.get("name", ""),
                    "value": attr_map.get("value", ""),
                    "type": attr_map.get("type", "text").lower(),
                }
            )

    def handle_endtag(self, tag: str) -> None:
        if tag == "form":
            self._current = None


class SourcegraphClient:
    def __init__(
        self,
        *,
        base_url: str,
        access_token: str | None = None,
        username: str | None = None,
        email: str | None = None,
        password: str | None = None,
    ) -> None:
        self.base_url = base_url.rstrip("/")
        self.access_token = access_token
        self.username = username
        self.email = email
        self.password = password
        self._cookie_jar = cookiejar.CookieJar()
        self._opener = urllib.request.build_opener(
            urllib.request.HTTPCookieProcessor(self._cookie_jar)
        )

    def wait_until_ready(self, timeout_seconds: int = 900) -> None:
        deadline = time.time() + timeout_seconds
        last_error = "unknown error"

        while time.time() < deadline:
            try:
                request = urllib.request.Request(f"{self.base_url}/")
                with self._opener.open(request, timeout=10) as response:
                    if response.status < 500:
                        return
            except (urllib.error.URLError, TimeoutError, socket.timeout) as exc:
                last_error = str(exc)

            time.sleep(5)

        raise SourcegraphError(f"timed out waiting for {self.base_url}: {last_error}")

    def ensure_authenticated(self) -> dict[str, Any]:
        user = self.current_user()
        if user is not None:
            return user

        if self.access_token:
            raise SourcegraphError("Sourcegraph access token was rejected")

        if not self.username or not self.password:
            raise SourcegraphError("missing Sourcegraph credentials")

        context = self.page_context("/sign-in")
        if context.get("needsSiteInit"):
            self.site_init()
        elif context.get("allowSignup"):
            self.sign_up()
        else:
            self.sign_in(context=context)

        user = self.current_user()
        if user is not None:
            return user

        raise SourcegraphError("could not authenticate with builtin auth")

    def current_user(self) -> dict[str, Any] | None:
        query = """
        query CurrentUser {
          currentUser {
            username
          }
        }
        """
        try:
            data = self.graphql(query, require_auth=False)
        except SourcegraphError:
            return None

        return data.get("currentUser")

    def list_repositories(self, *, query: str, limit: int) -> dict[str, Any]:
        data = self.graphql(
            """
            query ListRepos($query: String!, $first: Int!) {
              repositories(query: $query, first: $first) {
                nodes {
                  name
                  url
                }
                pageInfo {
                  hasNextPage
                  endCursor
                }
              }
            }
            """,
            {"query": query, "first": limit},
        )
        return data["repositories"]

    def read_file(
        self, *, repo: str, path: str, revision: str | None = None
    ) -> dict[str, Any]:
        data = self.graphql(
            """
            query ReadFile($repo: String!, $rev: String, $path: String!) {
              repository(name: $repo) {
                name
                commit(rev: $rev) {
                  oid
                  blob(path: $path) {
                    __typename
                    ... on GitBlob {
                      byteSize
                      isBinary
                      content
                    }
                  }
                }
              }
            }
            """,
            {"repo": repo, "rev": revision, "path": path},
        )
        return data["repository"]

    def list_directory(
        self, *, repo: str, path: str, revision: str | None = None
    ) -> dict[str, Any]:
        data = self.graphql(
            """
            query ListDirectory($repo: String!, $rev: String, $path: String!) {
              repository(name: $repo) {
                name
                commit(rev: $rev) {
                  oid
                  tree(path: $path) {
                    entries {
                      name
                      path
                      isDirectory
                    }
                  }
                }
              }
            }
            """,
            {"repo": repo, "rev": revision, "path": path},
        )
        return data["repository"]

    def search_code(self, *, query: str, display: int) -> dict[str, Any]:
        return self._graphql_or_stream_search(query=query, display=display)

    def search_commits(self, *, query: str, display: int) -> dict[str, Any]:
        commit_query = query if "type:" in query else f"{query} type:commit"
        return self._graphql_or_stream_search(query=commit_query, display=display)

    def graphql(
        self,
        query: str,
        variables: dict[str, Any] | None = None,
        *,
        require_auth: bool = True,
    ) -> dict[str, Any]:
        if require_auth:
            self.ensure_authenticated_if_needed()
        payload = json.dumps({"query": query, "variables": variables or {}}).encode(
            "utf-8"
        )
        headers = {
            "Content-Type": "application/json",
            "Accept": "application/json",
        }
        body = self._request("POST", "/.api/graphql", payload=payload, headers=headers)
        data = json.loads(body.decode("utf-8"))

        if data.get("errors"):
            raise SourcegraphError(str(data["errors"]))

        return data["data"]

    def search_stream(self, *, query: str, display: int) -> list[dict[str, Any]]:
        self.ensure_authenticated_if_needed()
        params = urllib.parse.urlencode(
            {
                "q": query,
                "v": "V3",
                "t": "literal",
                "display": str(display),
                "cm": "t",
                "cl": "1",
            }
        )
        headers = {"Accept": "text/event-stream"}
        request = self._build_request(
            "GET",
            f"/.api/search/stream?{params}",
            headers=headers,
        )
        events: list[dict[str, Any]] = []

        with self._opener.open(request, timeout=120) as response:
            event_type = "message"
            data_lines: list[str] = []

            for raw_line in response:
                line = raw_line.decode("utf-8").rstrip("\n")
                if line.startswith("event:"):
                    event_type = line.partition(":")[2].strip() or "message"
                    continue
                if line.startswith("data:"):
                    data_lines.append(line.partition(":")[2].lstrip())
                    continue
                if line:
                    continue
                if not data_lines:
                    event_type = "message"
                    continue
                data = "\n".join(data_lines)
                try:
                    parsed = json.loads(data)
                except json.JSONDecodeError:
                    parsed = {"raw": data}
                events.append({"event": event_type, "data": parsed})
                event_type = "message"
                data_lines = []

        return events

    def ensure_authenticated_if_needed(self) -> None:
        if self.access_token:
            return
        self.ensure_authenticated()

    def page_context(self, path: str) -> dict[str, Any]:
        html = self._request("GET", path).decode("utf-8", errors="replace")
        match = re.search(r"window\.context = (\{.*?\})\s*window\.pageError", html, re.S)
        if not match:
            raise SourcegraphError(f"window.context not found on {path}")
        return json.loads(match.group(1))

    def sign_in(self, *, context: dict[str, Any] | None = None) -> None:
        context = context or self.page_context("/sign-in")
        identifier = self.username or self.email or ""
        self._json_request(
            "/-/sign-in",
            {
                "email": identifier,
                "password": self.password or "",
            },
            xhr_headers=context.get("xhrHeaders"),
        )

    def sign_up(self, *, context: dict[str, Any] | None = None) -> None:
        if not self.email:
            raise SourcegraphError("missing Sourcegraph admin email for sign-up")
        context = context or self.page_context("/sign-up")
        self._json_request(
            "/-/sign-up",
            {
                "email": self.email,
                "username": self.username or self.email,
                "password": self.password or "",
            },
            xhr_headers=context.get("xhrHeaders"),
        )

    def site_init(self, *, context: dict[str, Any] | None = None) -> None:
        if not self.email:
            raise SourcegraphError("missing Sourcegraph admin email for site init")
        context = context or self.page_context("/site-init")
        self._json_request(
            "/-/site-init",
            {
                "email": self.email,
                "username": self.username or self.email,
                "password": self.password or "",
            },
            xhr_headers=context.get("xhrHeaders"),
        )

    def _graphql_or_stream_search(self, *, query: str, display: int) -> dict[str, Any]:
        try:
            data = self.graphql(
                """
                query Search($query: String!) {
                  search(query: $query) {
                    results {
                      limitHit
                      results {
                        __typename
                        ... on FileMatch {
                          repository {
                            name
                          }
                          file {
                            path
                            url
                          }
                          lineMatches {
                            preview
                            lineNumber
                          }
                        }
                        ... on CommitSearchResult {
                          label
                          url
                          detail
                        }
                      }
                    }
                  }
                }
                """,
                {"query": query},
            )
            return data["search"]["results"]
        except SourcegraphError:
            return {"events": self.search_stream(query=query, display=display)}

    def _attempt_form_auth(self, path: str) -> bool:
        html = self._request("GET", path).decode("utf-8", errors="replace")
        parser = _FormParser()
        parser.feed(html)

        form = None
        for candidate in parser.forms:
            action = candidate["action"] or path
            if path.strip("/") in action or any(
                entry["type"] == "password" for entry in candidate["inputs"]
            ):
                form = candidate
                break

        if form is None:
            return False

        payload: dict[str, str] = {}
        for entry in form["inputs"]:
            if entry["name"]:
                payload[entry["name"]] = entry["value"]

        username_field = self._pick_field(
            form["inputs"], ["username", "email", "login", "identifier"]
        )
        password_field = self._pick_field(form["inputs"], ["password"], input_type="password")
        email_field = self._pick_field(form["inputs"], ["email"])

        if username_field:
            payload[username_field] = self.username or self.email or ""
        if email_field and self.email:
            payload[email_field] = self.email
        if password_field:
            payload[password_field] = self.password or ""
        for entry in form["inputs"]:
            name = entry["name"]
            if not name:
                continue
            lowered = name.lower()
            if "confirm" in lowered and "password" in lowered:
                payload[name] = self.password or ""

        action = form["action"] or path
        self._request(
            form["method"].upper(),
            action,
            payload=urllib.parse.urlencode(payload).encode("utf-8"),
            headers={
                "Content-Type": "application/x-www-form-urlencoded",
                "Referer": f"{self.base_url}{path}",
            },
        )
        return True

    def _json_request(
        self,
        path: str,
        payload: dict[str, Any],
        *,
        xhr_headers: dict[str, str] | None = None,
    ) -> None:
        headers = {
            "Accept": "application/json",
            "Content-Type": "application/json",
        }
        if xhr_headers:
            headers.update(xhr_headers)
        self._request(
            "POST",
            path,
            payload=json.dumps(payload).encode("utf-8"),
            headers=headers,
        )

    def _pick_field(
        self,
        inputs: list[dict[str, str]],
        keywords: list[str],
        *,
        input_type: str | None = None,
    ) -> str | None:
        for entry in inputs:
            name = entry["name"]
            if not name:
                continue
            lowered = name.lower()
            if input_type and entry["type"] != input_type:
                continue
            if any(keyword in lowered for keyword in keywords):
                return name
        return None

    def _request(
        self,
        method: str,
        path: str,
        *,
        payload: bytes | None = None,
        headers: dict[str, str] | None = None,
    ) -> bytes:
        request = self._build_request(method, path, payload=payload, headers=headers)
        try:
            with self._opener.open(request, timeout=60) as response:
                return response.read()
        except urllib.error.HTTPError as exc:
            body = exc.read().decode("utf-8", errors="replace")
            raise SourcegraphError(f"{exc.code} {exc.reason}: {body[:400]}") from exc
        except urllib.error.URLError as exc:
            raise SourcegraphError(str(exc.reason)) from exc

    def _build_request(
        self,
        method: str,
        path: str,
        *,
        payload: bytes | None = None,
        headers: dict[str, str] | None = None,
    ) -> urllib.request.Request:
        url = path if path.startswith("http://") or path.startswith("https://") else f"{self.base_url}{path}"
        request = urllib.request.Request(url=url, data=payload, method=method)
        request.add_header("User-Agent", "sourcegraph-local-bridge/0.1")
        for key, value in (headers or {}).items():
            request.add_header(key, value)
        if self.access_token:
            request.add_header("Authorization", f"token {self.access_token}")
        return request
