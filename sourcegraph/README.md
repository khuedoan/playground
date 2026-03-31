# Sourcegraph Docker Compose on Colima

This experiment tracks a local Sourcegraph deployment based on the upstream
Docker Compose layout for Sourcegraph `7.0.2852`, with a small local override
for Colima and a file-backed site config for reproducible local auth.

## Objective

Run the upstream stack locally on Colima with a smaller override while keeping
the core services reachable and healthy.

Primary metric:

- `healthy_core_services`
- Unit: service count
- Higher is better

## Current Result

Validated on March 30, 2026:

- `docker compose config` renders cleanly with the local override.
- `http://127.0.0.1/` responds with `302` to `/sign-in`, so the frontend is
  reachable through Caddy.
- `pgsql`, `codeintel-db`, and `zoekt-webserver-0` are healthy after applying
  the local fixes in `compose.override.yaml`.
- Grafana provisions the local datasources without keeping the stale Jaeger
  datasource.
- `site-config.json` is applied through `SITE_CONFIG_FILE`, so Sourcegraph now
  advertises `http://127.0.0.1:7080` as `externalURL` instead of the
  unconfigured placeholder URL.
- Builtin auth works again on the local HTTP endpoint.
- The local MCP bridge can authenticate with Sourcegraph and `list_repos`
  succeeds. It currently returns no repositories because code hosts have not
  been configured yet.

Known issue:

- `cadvisor` still restarts on ARM Colima because it cannot detect CPU clock
  speed from the Colima guest's `/proc/cpuinfo`. Container-level Docker metrics
  remain a separate compatibility issue.

## Reproduce

From [`sourcegraph`](/Users/khuedoan/Documents/playground/sourcegraph):

```bash
docker compose up -d --build
./bin/init-sourcegraph.sh
curl -I http://127.0.0.1/
docker compose ps
```

To stop the stack:

```bash
docker compose down --remove-orphans
colima stop
```
