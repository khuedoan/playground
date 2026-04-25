# OpenBao + SecretSpec Bootstrap (Simple)

## Quick Summary (How Everything Works)
`./scripts/bootstrap.sh` is the single entrypoint. On each run it:

1. Finds an OpenBao CLI (`BAO_BIN`, PATH, or `./.tools/bin/bao`), and auto-installs one if missing.
2. Connects to `BAO_ADDR`; if unreachable and `START_DEV=1`, it starts a local dev server.
3. Ensures the configured KV v2 mount exists (default: `apps`).
4. Loads `bootstrap-secrets.json` and processes each field by strategy:
   - `literal`: use value as-is,
   - `generate`: create value at runtime (`password` or `uuid`),
   - `env`: read from environment variable or prompt for paste input.
5. Writes each secret path into OpenBao with `kv put`.

The smoke test (`./scripts/smoke-test.sh`) runs this flow non-interactively with test env vars, then reads back values to verify bootstrap correctness.

## Question
How can we bootstrap a new OpenBao cluster with **one command**, including both:
- randomly generated secrets, and
- third-party secrets that must be copy-pasted?

## Setup
This experiment provides:

- Nix dev shell for required tools (`openbao`, `just`, `jq`, `yq`).
- Local fallback installer for OpenBao CLI into `./.tools/bin/bao`.
- One bootstrap script (`scripts/bootstrap.sh`) that:
  - resolves/install OpenBao CLI,
  - starts local dev cluster when needed,
  - enables kv-v2 mount,
  - imports a mixed secret plan from `bootstrap-secrets.json`.
- One smoke test that verifies round-trip reads.

## Secret Plan Format
`bootstrap-secrets.json` supports three field types:

- `literal`: fixed value committed in file (safe only for non-sensitive values).
- `generate`: generated at bootstrap time (`password` or `uuid`).
- `env`: pulled from environment variable, with optional interactive prompt (copy/paste flow).

Example entries in this repo:

- `apps/payments/api`
  - `username` from `literal`
  - `password` from `generate`
  - `endpoint` from `env` (`PAYMENTS_ENDPOINT`)
- `apps/thirdparty/stripe`
  - `api_key` + `webhook_secret` from `env`
  - `rotation_id` from generated `uuid`

## One-command bootstrap
### Brand-new local cluster
```bash
./scripts/bootstrap.sh
```

### Existing cluster (no dev server start)
```bash
BAO_ADDR=https://bao.example.com \
BAO_TOKEN=<root-or-bootstrap-token> \
START_DEV=0 \
./scripts/bootstrap.sh
```

### Non-interactive copy-paste inputs
Set required env vars before running the same one command:

```bash
export PAYMENTS_ENDPOINT='https://api.example.internal'
export STRIPE_API_KEY='sk_live_...'
export STRIPE_WEBHOOK_SECRET='whsec_...'
./scripts/bootstrap.sh
```

## Reproduce Validation
```bash
./scripts/install-openbao.sh
BAO_BIN=./.tools/bin/bao ./scripts/smoke-test.sh
```

## Files
- `bootstrap-secrets.json`: one-file secret bootstrap plan
- `scripts/install-openbao.sh`: local installer fallback
- `scripts/bootstrap.sh`: one-command bootstrap
- `scripts/smoke-test.sh`: runtime verification
- `secretspec.example.yaml`: downstream SecretSpec template
- `justfile`: convenience commands
- `notes.md`: append-only experiment log
