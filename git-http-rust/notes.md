# Notes

## Objective
Implement OAuth2/OIDC-style auth directly (phase B direction), ship a `gx` CLI login flow, and add basic web repository browsing while keeping git clone/push/pull working.

## Primary metric
- **Metric:** authenticated end-to-end success rate
- **Unit:** successful `gx login + clone + push + pull + UI browse` runs / total runs
- **Target direction:** higher is better (aim 100%)

## Iteration log
- Added OAuth2 Device Flow endpoints to the server (`/oauth/device/code`, `/oauth/token`, `/oauth/verify`).
- Added JWT auth verification middleware logic for git and UI endpoints.
- Added scope enforcement (`repo:read`, `repo:write`) with write checks for receive-pack.
- Added `gx` CLI binary for device login and token retrieval.
- Added basic web UI pages to list repos, browse trees, and view blob contents.
- Updated e2e script to validate auth + git operations + UI listing.

## Results
- End-to-end scenario succeeds with `make e2e`.
- Authenticated clone/push/pull works with token-as-password in Git Basic auth.
- Basic repo browser works using access token.

## Decisions
- Implement phase B-style OAuth2 device flow directly in this prototype.
- Keep external OIDC federation as a follow-up hardening step.
