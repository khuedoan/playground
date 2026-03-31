# Notes

## 2026-03-28

- Imported the upstream Sourcegraph `7.0.2852` Docker Compose stack into this
  experiment folder with a smaller `compose.override.yaml`.
- Fixed the local Postgres memory limits so `pgsql` and `codeintel-db` have
  headroom above their configured `shared_buffers = 1GB`.
- Removed `trusted_proxies 0.0.0.0/0` from all built-in Caddy templates so the
  default local stack does not trust spoofed `X-Forwarded-*` headers.
- Removed the broken Jaeger datasource from Grafana provisioning and added
  `deleteDatasources` so an already-provisioned stale datasource is deleted on
  restart.
- Found an additional Colima-specific runtime issue: `zoekt-webserver-0`
  failed because the image starts it through `/sbin/tini -s`. The local
  override now invokes `zoekt-webserver` directly with the same flags.
- Verified:
  - `docker compose config`
  - `docker compose up -d`
  - `docker compose restart caddy grafana`
  - `curl -I http://127.0.0.1/`
  - `docker compose exec zoekt-webserver-0 wget -q 'http://127.0.0.1:6070/healthz' -O -`
- Tried disabling cadvisor CPU metrics with
  `-disable_metrics=cpu,cpuLoad,percpu`, but it still failed with the same CPU
  clock-speed detection error on ARM Colima.

## 2026-03-30

- Found that Sourcegraph `7.0.2852` no longer exposes usable builtin auth HTML
  forms. The sign-in and sign-up pages are SPA routes that POST JSON to
  `/-/sign-in`, `/-/sign-up`, and `/-/site-init`.
- Confirmed the old bootstrap path failed even with valid admin credentials
  because the instance was still advertising
  `https://unconfigured.sourcegraph.com` as `externalURL`, which caused
  `/-/sign-in` to issue a `Secure` session cookie that was never reused on the
  local HTTP endpoint.
- Added a repo-controlled `site-config.json` and mounted it into both frontend
  containers via `SITE_CONFIG_FILE`. The file must include both
  `"externalURL": "http://127.0.0.1:7080"` and an explicit builtin auth
  provider, or the frontend refuses to start with "no auth providers set".
- Updated `mcp-bridge/sourcegraph_client.py` to authenticate using the current
  JSON auth endpoints instead of scraping forms.
- Verified from inside the running bridge container:
  - `/sign-in` now reports `externalURL = "http://127.0.0.1:7080"`
  - `/-/sign-in` returns a non-secure `sgs` cookie for local HTTP
  - GraphQL `currentUser` returns `khue`
  - `SourcegraphClient.ensure_authenticated()` succeeds
  - MCP `list_repos` succeeds and currently returns an empty page before any
    code hosts are configured
