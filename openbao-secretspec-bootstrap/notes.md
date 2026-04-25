# Notes

## 2026-04-25

### Objective
Build a one-command OpenBao bootstrap for a new cluster that can handle both generated secrets and copy-pasted third-party secrets.

### Baseline
Initial version could seed only a fixed sample secret and did not model manual secret intake explicitly.

### What I tried
1. Reworked `scripts/bootstrap.sh` into a one-command orchestrator.
2. Added auto-install fallback for `bao` CLI when missing (`AUTO_INSTALL_BAO=1`).
3. Added JSON-driven secret plan (`bootstrap-secrets.json`) with per-field strategies:
   - `literal`
   - `generate` (`password` / `uuid`)
   - `env` (env var or interactive prompt)
4. Added `bootstrap-existing` flow (`START_DEV=0`) for remote/pre-existing clusters.
5. Expanded smoke test to validate both generated and env-provided secrets.

### Key metric
- Bootstrap lead time: not measured yet (seconds, lower is better).

### Results
- End-to-end smoke test passed with local OpenBao v2.5.3.
- Confirmed one-command bootstrap can seed multiple secret paths with mixed generation and copy-paste sources.

### Failed attempts / issues
- `nix`/`just` not available in this environment, so validation used direct shell scripts and local installer.

### Decision
Keep JSON + jq for portability in minimal environments; defer advanced policy/app-role bootstrap to a follow-up iteration.
