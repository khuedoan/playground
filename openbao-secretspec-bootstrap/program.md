# Session Program: OpenBao + SecretSpec Bootstrap

## Objective
Create a minimal, reproducible bootstrap that runs OpenBao in dev mode, seeds one secret, and stores a matching SecretSpec manifest template.

## Primary Metric
- **Metric name:** Bootstrap lead time
- **Unit:** seconds
- **Direction:** lower is better

## Constraints
- Keep all artifacts inside this experiment directory.
- Prefer plain shell scripts and simple YAML.
- Avoid external orchestration beyond local CLI tooling.

## Files in Scope
- `flake.nix`
- `.envrc`
- `justfile`
- `scripts/bootstrap.sh`
- `secretspec.example.yaml`
- `notes.md`
- `README.md`

## Stop Conditions
- Bootstrap script starts OpenBao dev server instructions successfully.
- One sample secret path and values are defined.
- SecretSpec template references the same path and keys.
- Reproduction steps are documented.
