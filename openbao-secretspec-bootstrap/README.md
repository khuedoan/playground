# OpenBao + SecretSpec Bootstrap (Hard Requirement: secretspec.dev)

This experiment now uses **SecretSpec CLI from https://secretspec.dev** as the required interface for seeding and reading secrets.

## How it works
`./scripts/bootstrap.sh` is the single command. It:

1. Installs/resolves `bao` and `secretspec` CLIs.
2. Starts OpenBao dev mode if needed (`START_DEV=1`).
3. Uses SecretSpec's **OpenBao provider URI** (`openbao://.../apps?tls=false`) to write secrets.
4. Loads secret strategy entries from `bootstrap-secrets.json` (`literal`, `env`, `generate`).
5. Calls `secretspec set ... --provider openbao://...` for each key.
6. Runs `secretspec check` to verify required declarations from `secretspec.toml`.

## Files
- `secretspec.toml`: SecretSpec declarations (project + required keys)
- `bootstrap-secrets.json`: bootstrap strategies for each key
- `scripts/bootstrap.sh`: one-command bootstrap through SecretSpec
- `scripts/smoke-test.sh`: end-to-end verification with `secretspec get`
- `scripts/install-openbao.sh`: local bao installer
- `scripts/install-secretspec.sh`: local secretspec installer

## One-command usage
### Local new cluster
```bash
./scripts/bootstrap.sh
```

### Existing OpenBao cluster
```bash
BAO_ADDR=https://bao.example.com \
BAO_TOKEN=<bootstrap-token> \
START_DEV=0 \
./scripts/bootstrap.sh
```

### Third-party copy/paste values (non-interactive)
```bash
export PAYMENTS_ENDPOINT='https://api.example.internal'
export STRIPE_API_KEY='sk_live_...'
export STRIPE_WEBHOOK_SECRET='whsec_...'
./scripts/bootstrap.sh
```

## Validation
```bash
./scripts/smoke-test.sh
```

The smoke test verifies values using:
- `secretspec get PAYMENTS_USERNAME --provider openbao://...`
- `secretspec get STRIPE_API_KEY --provider openbao://...`
