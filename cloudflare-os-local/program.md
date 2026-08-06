# Session

## Objective

Run Cloudflare OS locally on the open-source `workerd` runtime through Wrangler,
without installing its Node.js toolchain directly on the host.

## Primary metric

- **Name:** local HTTP readiness
- **Unit:** successful HTTP response at `/`
- **Direction:** higher is better (target: 1 of 1 requests succeeds)

## Constraints and scope

- Files in scope: this directory only.
- Pin and verify the upstream source archive.
- Use Docker Compose for runtime isolation and Nix for host tooling.
- Persist Wrangler state between container restarts.

## Stop conditions

Stop when Compose validates, the image builds, and `http://localhost:8787/`
responds, or document an environmental blocker in `notes.md`.
