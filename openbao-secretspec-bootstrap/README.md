# OpenBao + SecretSpec Bootstrap (Simple)

## Question
How can we bootstrap a tiny local OpenBao setup and pair it with a matching SecretSpec manifest in the smallest reproducible workflow?

## Setup
This experiment provides:

- Nix dev shell for required tools (`openbao`, `just`, `jq`, `yq`).
- One bootstrap script to start OpenBao in dev mode and write a sample secret.
- One `SecretSpec` example that maps keys from the OpenBao path into a target secret.

Files:

- `flake.nix` / `.envrc`: development environment
- `scripts/bootstrap.sh`: one-command bootstrap
- `secretspec.example.yaml`: manifest template
- `justfile`: convenience commands
- `notes.md`: append-only experiment log

## Results
- Created a baseline bootstrap flow and aligned SecretSpec mapping.
- Runtime execution in this environment was not validated yet.

## Reproduce
From this directory:

```bash
direnv allow   # optional if using direnv
nix develop
just bootstrap
```

Useful follow-ups:

```bash
just status
just show-secret
just stop
```

## Expected Output
After `just bootstrap`, OpenBao should contain:

- path: `apps/data/payments/api`
- keys:
  - `username`
  - `password`
  - `endpoint`

`secretspec.example.yaml` references this exact path and keys.

## Next Steps
- Measure and record bootstrap lead time (seconds).
- Add policy + non-root token bootstrap.
- Optionally add containerized runner for environments without native OpenBao.
