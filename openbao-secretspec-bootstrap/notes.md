# Notes

## 2026-04-25

### Objective
Build a minimal OpenBao + SecretSpec bootstrap that is easy to rerun locally.

### Baseline
No experiment directory or bootstrap assets existed yet.

### What I tried
1. Created a new isolated experiment directory.
2. Added a Nix flake dev shell with OpenBao and utility CLIs.
3. Added a shell bootstrap script that:
   - starts OpenBao dev mode,
   - seeds one kv-v2 secret,
   - prints redacted output.
4. Added a SecretSpec example manifest aligned to the seeded secret path.
5. Added a `justfile` for common run/status/stop tasks.
6. Added a local installer (`scripts/install-openbao.sh`) for environments without Nix.
7. Added a smoke test (`scripts/smoke-test.sh`) that boots OpenBao and validates round-trip reads.

### Key metric
- Bootstrap lead time: **not measured yet** (seconds, lower is better).

### Results
- Smoke test succeeded end-to-end with OpenBao `v2.5.3` binary installed in `./.tools/bin/bao`.

### Failed attempts / issues
- `nix` and `just` were not preinstalled in the environment, so the fallback installer path was needed for runtime validation.

### Decision
Keep this first version intentionally small and text-based; defer advanced auth/policy automation until baseline execution is confirmed.
