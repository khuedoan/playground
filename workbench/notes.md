# Work log

## 2026-08-10 — baseline

- Recovered the original MVP from commit `81e5887`.
- Baseline checks from the original experiment: 7 Rust tests and 4 Phoenix/PostgreSQL tests passed; runtime validation was blocked by the originating sandbox.
- Decision: make `microvm.nix` the only production provisioning backend and define the developer environment directly in NixOS.
- Primary metric: production provisioning backends, count, lower is better. Baseline: multiple exposed choices; target: 1 production backend (`microvm.nix`) with test doubles kept private to tests.
- Secondary metric: obsolete production artifacts, count, lower is better. Baseline: 7; target: 0.
- Constraint: this Work sandbox has no KVM, `/proc`, cgroups, or Nix daemon, so it can validate source/tests but cannot boot a MicroVM. A NixOS/KVM CI or host run remains the release gate.

## 2026-08-10 — real GitHub Actions end-to-end gate

- Added a KVM job that installs the official MicroVM systemd service topology on the ephemeral Linux runner, then exercises the normal Phoenix-to-host-agent provisioning path.
- Pi uses the real Qwen2.5 Coder GGUF through a guest-local llama.cpp server, so the gate needs no model-provider secret and sends no prompt outside the MicroVM.
- The gate requires a real guest boot, Pi tool use, code-server, Sway, wayvnc, noVNC, Blender, and persistence across a MicroVM restart.
- Playwright records the real UI session and the workflow uploads the video together with systemd and network diagnostics.
