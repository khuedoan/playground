# Notes

## 2026-04-26

### Objective
Meet hard requirement to use `https://secretspec.dev` directly (not just a placeholder spec file) for OpenBao bootstrap.

### Changes
- Added `secretspec.toml` declarations for required keys.
- Added `scripts/install-secretspec.sh` for local SecretSpec CLI install.
- Reworked `scripts/bootstrap.sh` to write secrets via `secretspec set --provider openbao://...`.
- Reworked `scripts/smoke-test.sh` to verify via `secretspec get`.

### Result
Bootstrap and smoke test now exercise SecretSpec + OpenBao provider end-to-end.
