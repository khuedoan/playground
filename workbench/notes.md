# Work log

## 2026-08-10 — baseline

- Recovered the Docker-backed MVP from commit `81e5887`.
- Baseline checks from the original experiment: 7 Rust tests and 4 Phoenix/PostgreSQL tests passed; Docker runtime validation was blocked by the originating sandbox.
- Decision: remove Docker as a host provisioning backend and make `microvm.nix` the only production backend. Docker remains available *inside* each NixOS guest because that is part of the product requirement.
- Primary metric: host provisioning backends, count, lower is better. Baseline: 2 exposed choices (`docker`, `mock`); target: 1 production backend (`microvm.nix`) with test doubles kept private to tests.
- Secondary metric: Docker-specific production files, count, lower is better. Baseline: 7 (`compose.yaml`, four Dockerfiles, entrypoint, Docker-oriented demo script); target: 0.
- Constraint: this Work sandbox has no KVM, `/proc`, cgroups, or Nix daemon, so it can validate source/tests but cannot boot a MicroVM. A NixOS/KVM CI or host run remains the release gate.
