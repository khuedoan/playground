# Session Program: OpenBao + SecretSpec Bootstrap

## Objective
Create a one-command OpenBao bootstrap that seeds secrets for a new or existing cluster, including both generated and copy-pasted third-party values.

## Primary Metric
- **Metric name:** Bootstrap lead time
- **Unit:** seconds
- **Direction:** lower is better

## Constraints
- Keep all artifacts inside this experiment directory.
- Prefer plain shell + JSON for portability.
- Make the bootstrap usable in non-Nix environments too.

## Files in Scope
- `bootstrap-secrets.json`
- `flake.nix`
- `.envrc`
- `justfile`
- `scripts/install-openbao.sh`
- `scripts/bootstrap.sh`
- `scripts/smoke-test.sh`
- `secretspec.example.yaml`
- `notes.md`
- `README.md`

## Stop Conditions
- Single command bootstraps a local cluster and seeds secrets.
- Same command works for existing clusters via env vars.
- Secret plan supports generated and copy-pasted values.
- Smoke test validates round-trip reads on seeded paths.
