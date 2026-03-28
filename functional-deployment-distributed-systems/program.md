# Program: Functional Deployment Model for Distributed Systems

## Objective
Design and document a **functional deployment model** that generalizes single-node Nix-style deployments to multi-node distributed systems with explicit semantics for safety, convergence, and rollback.

## Primary metric
- **Metric name:** Mean Time To Convergence (MTTC)
- **Unit:** seconds
- **Better direction:** lower is better

## Secondary metrics
- Rollback latency (seconds, lower is better)
- Failed rollout rate (percent, lower is better)
- Configuration drift incidence (count per week, lower is better)

## Constraints
- Keep this experiment self-contained in this directory.
- Use prior art grounded in functional/declarative systems.
- Emphasize properties that are testable and operationally useful.
- Produce a resumable artifact set (`notes.md`, `README.md`).

## Files in scope
- `functional-deployment-distributed-systems/program.md`
- `functional-deployment-distributed-systems/notes.md`
- `functional-deployment-distributed-systems/README.md`
- `functional-deployment-distributed-systems/flake.nix`
- `functional-deployment-distributed-systems/.envrc`

## Stop conditions
- Prior art is summarized with strengths and gaps.
- A concrete model is specified (state, transitions, invariants).
- Reproduction/documentation is sufficient for a fresh agent to continue.
