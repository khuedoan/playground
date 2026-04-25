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

### Key metric
- Bootstrap lead time: **not measured yet** (seconds, lower is better).

### Failed attempts / issues
- Did not execute OpenBao locally in this environment, so runtime compatibility of the `openbao` Nix package is unverified.

### Decision
Keep this first version intentionally small and text-based; defer advanced auth/policy automation until baseline execution is confirmed.
