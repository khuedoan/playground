# OpenBao + SecretSpec Bootstrap (Simple)

## Question
How can we bootstrap a tiny local OpenBao setup and pair it with a matching SecretSpec manifest in the smallest reproducible workflow?

## Setup
This experiment provides:

- Nix dev shell for required tools (`openbao`, `just`, `jq`, `yq`).
- Local fallback installer for OpenBao CLI into `./.tools/bin/bao`.
- One bootstrap script to start OpenBao in dev mode and write a sample secret.
- One `SecretSpec` example that maps keys from the OpenBao path into a target secret.
- One smoke test that verifies values can be read back.

Files:

- `flake.nix` / `.envrc`: development environment
- `scripts/install-openbao.sh`: local installer fallback
- `scripts/bootstrap.sh`: one-command bootstrap
- `scripts/smoke-test.sh`: runtime verification
- `secretspec.example.yaml`: manifest template
- `justfile`: convenience commands
- `notes.md`: append-only experiment log

## Results
- Created a baseline bootstrap flow and aligned SecretSpec mapping.
- Verified bootstrap behavior with real OpenBao runtime using the smoke test.

## Reproduce
From this directory (without Nix):

```bash
./scripts/install-openbao.sh
BAO_BIN=./.tools/bin/bao ./scripts/smoke-test.sh
```

From this directory (with Nix):

```bash
direnv allow   # optional if using direnv
nix develop
just smoke-test
```

Useful follow-ups:

```bash
just status
just show-secret
just stop
```

## Expected Output
After `bootstrap`/`smoke-test`, OpenBao should contain:

- path: `apps/data/payments/api`
- keys:
  - `username`
  - `password`
  - `endpoint`

`secretspec.example.yaml` references this exact path and keys.

## Next Steps
- Measure and record bootstrap lead time (seconds).
- Add policy + non-root token bootstrap.
- Optionally add TLS-enabled local profile.
