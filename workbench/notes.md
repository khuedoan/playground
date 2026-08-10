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

## 2026-08-10 — deterministic E2E model API

- Replaced Qwen download and llama.cpp inference in the GitHub Actions E2E configuration with a deterministic OpenAI-compatible API inside the real MicroVM.
- Kept the production default unchanged: normal generated guests still use the pinned local Qwen model and llama.cpp.
- The mock emits a real Pi `bash` tool call, Pi executes the requested command, and a second streamed completion returns the expected assistant response. MicroVM boot, Phoenix reconciliation, Pi RPC, tool execution, GUI services, recording, and restart persistence remain covered.
- E2E model artifact downloads: 1 GGUF to 0, lower is better. The browser agent timeout was reduced from 900 seconds to 60 seconds.
- Full timing still requires the KVM GitHub Actions runner; this sandbox cannot boot the guest.

## 2026-08-10 — green real-MicroVM gate

- Run #34 showed that the mock flag was lost across `sudo env`, while Sway and noVNC lacked `dbus-daemon` and `ps` in their service paths.
- Runs #36 and #38 proved the mock Pi tool call, noVNC recording, and guest services worked; they also exposed Sway's missing shell path and Blender's Wayland/EGL exit.
- Runs #40 and #42 kept Blender alive through Mesa software OpenGL and Xwayland, then exposed two test-only issues: `pgrep` was absent globally and Nix's Blender wrapper does not use `blender` as its exact kernel process name.
- Run #44 passed Rust, Phoenix, Nix, and the complete KVM E2E job on commit `53a03a4`. The gate booted the real MicroVM, executed Pi's real `bash` tool through the deterministic mock API, verified the GUI/code-server/Blender processes, restarted the guest, and confirmed the file persisted.
- Successful browser artifact: H.264 at 1440×900, 25 fps, 152.84 seconds, 2,118,935 bytes. Final screenshot: 1440×1722, 249,489 bytes.
