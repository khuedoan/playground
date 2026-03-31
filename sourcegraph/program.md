# Program

## Objective

Stand up a single-user local code search stack that can index and search a large
mixed GitHub estate across GHES, GHEC, and public GitHub repositories, while
also exposing code search to local agents.

Primary metric:

- `searchable_repositories`
- Unit: repository count
- Higher is better

## Constraints

- Single operator
- Storage is not a constraint
- RAM is limited to 48 GB total on the host
- Setup should stay close to a single `docker compose up`
- No Sourcegraph Enterprise license key is available

## Files In Scope

- `compose.yaml`
- `compose.override.yaml`
- `site-config.json`
- `mcp-bridge/`
- `bin/`
- `README.md`
- `notes.md`
- `justfile`
- `flake.nix`
- `.envrc`
- `.env.example`

## Stop Conditions

- Sourcegraph boots locally behind a localhost-only port
- Admin bootstrap is scripted
- The MCP bridge can list repos, search code, and read files from Sourcegraph
- The experiment docs are complete enough for a fresh agent to resume
