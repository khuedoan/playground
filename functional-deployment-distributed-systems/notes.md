# Notes

## 2026-03-28

### Baseline
- Starting point: prior art reference from user is Dolstra's thesis on the purely functional software deployment model (single-node focus with distributed build/deployment implications).
- Baseline approach in many production systems today is either:
  - imperative orchestration (scripts/runbooks), or
  - declarative reconciliation without full functional closure guarantees.

### What I tried
1. Framed the problem as extending Nix's immutable closure + atomic profile switch from one host to a fleet.
2. Reviewed additional operationally relevant prior-art families:
   - NixOps-style multi-machine declarations.
   - GitOps reconciliation controllers (Flux/Argo CD) for desired-state convergence.
3. Drafted a model where deployment is an immutable graph + epoch pointer updates rather than mutable in-place edits.

### Key design decisions
- Use a **global deployment value** computed from a pure specification function.
- Make artifacts and node configurations **content-addressed** and immutable.
- Separate rollout into:
  1. realization (build/prefetch/prepare),
  2. activation (pointer switch),
  3. stabilization (health-gated wave progression).
- Represent rollout/rollback as movement between immutable **epochs**.

### Failed/weak ideas discarded
- Full distributed two-phase commit for every node at activation time:
  - too brittle in partitions,
  - over-couples availability to strict synchrony.
- Purely best-effort eventual convergence with no epoch notion:
  - weak auditability,
  - ambiguous rollback targets.

### Current hypothesis
A practical functional deployment model for distributed systems should combine:
- immutable closure semantics (Nix lineage),
- controller reconciliation loops (GitOps lineage),
- explicit epoch/wave health policies for safe progression.

### Next steps
- Validate model against concrete failure scenarios:
  - network partition during wave 2,
  - partial secret provisioning failure,
  - incompatible schema migration across mixed-version cluster.
- Add a simulation harness or pseudo-formal transition table if iteration continues.
