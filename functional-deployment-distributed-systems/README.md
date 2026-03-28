# Functional Deployment Model for Distributed Systems

## Research question
How can we extend the core ideas of the purely functional deployment model (as established for single-node systems) into a robust deployment model for distributed systems without losing reproducibility, rollbackability, and operational safety?

## Prior art snapshot

### 1) Purely functional deployment model (Nix)
- Core contribution: deployments are built from immutable, content-addressed closures; upgrades are pointer switches to new generations.
- Operational value: atomic activation and reliable rollback at the node level.
- Gap for distributed systems: the thesis gives strong foundations, but distributed activation semantics (waves, health gating, inter-node dependencies, schema coordination) are not the primary abstraction boundary.

Reference:
- Eelco Dolstra, *The Purely Functional Software Deployment Model* (PhD thesis, 2006): https://edolstra.github.io/pubs/phd-thesis.pdf

### 2) NixOps-style multi-machine declarations
- Core contribution: network-level declarations where machine configs are derived from one Nix expression and can reference sibling node configuration.
- Operational value: cross-node consistency from a single source of truth.
- Gap: rollout safety logic is mostly operator policy rather than first-class semantic objects.

Reference:
- NixOps overview: https://nixops.readthedocs.io/en/latest/overview.html

### 3) GitOps reconcilers (Flux / Argo CD lineage)
- Core contribution: continuous reconciliation of cluster state to declarative desired state from versioned Git artifacts.
- Operational value: drift correction and auditable change intent.
- Gap: these systems are declarative and convergent, but usually do not impose functional closure semantics across all runtime dependencies by default.

References:
- Flux concepts: https://fluxcd.io/flux/concepts/
- Argo CD docs: https://argo-cd.readthedocs.io/en/stable/

---

## Proposed model: Functional Distributed Deployment Model (FDDM)

### Design goals
1. Preserve Nix-like immutable closure semantics.
2. Add explicit distributed rollout semantics.
3. Keep recovery simple: rollback by epoch pointer, not ad hoc repair.
4. Maintain eventual convergence under partial failure.

### Data model
- **Spec** `S`: versioned desired system specification (topology, services, policies, rollout constraints).
- **Compiler** `C`: pure function from `S` and inputs to deployment plan.
- **Epoch** `E_k`: immutable deployment snapshot with:
  - per-node closure references,
  - rollout graph,
  - dependency constraints,
  - health predicates,
  - migration actions.
- **Runtime state** `R`: observed node health, currently active epoch pointers, progress markers.

Functional view:
- `Plan = C(S, Inputs)`
- `E_k = materialize(Plan)`
- Deployment is transition of pointers from `E_(k-1)` to `E_k` under policy.

### Execution model

#### Phase 1: Realization (side-effect controlled)
- Build/fetch all required closures.
- Validate artifact integrity and signatures.
- Preflight dependency checks (capacity, credentials, migration compatibility).

#### Phase 2: Activation (wave-based pointer switching)
- Partition nodes into ordered waves.
- For each wave:
  1. switch node pointer to `E_k` candidate,
  2. run readiness and invariants,
  3. if success threshold passes, advance wave; else rollback affected wave.

#### Phase 3: Stabilization (reconciliation loop)
- Controller continually reconciles drift back to `E_k`.
- If global SLO or invariant violations persist beyond threshold, demote to `E_(k-1)` or pause.

### Invariants
1. **Immutability:** epoch content never changes once published.
2. **Traceability:** every live node references an epoch ID and closure digest.
3. **Bounded divergence:** mixed-epoch operation is allowed only within policy-defined windows.
4. **Monotonic audit log:** all transitions append-only, with actor + reason.
5. **Deterministic rollback target:** rollback always points to a known prior epoch.

### Failure semantics
- **Node failure during wave:** hold/rollback only that wave unless policy escalates.
- **Partition:** continue in connected components only if safety predicates remain true; otherwise freeze progression.
- **Schema migration mismatch:** gate next wave until compatibility checks pass; require explicit dual-read/dual-write window when needed.

### Why this is “functional” (not only declarative)
- Declarative desired state alone is insufficient; FDDM additionally requires:
  - content-addressed, immutable realization artifacts,
  - pure compilation from spec to deployable graph,
  - epoch-based activation semantics with rollback by pointer.

---

## Evaluation plan

### Primary metric
- **MTTC (Mean Time To Convergence)** in seconds; lower is better.

### Secondary metrics
- Rollback latency.
- Failed rollout rate.
- Drift incidents per week.

### Suggested benchmark scenarios
1. Normal rollout across 3 waves and 100 nodes.
2. Wave-2 induced failure in 15% of nodes.
3. Network partition splitting control plane from one zone.
4. Backward-compatible database migration, then incompatible migration attempt.

### Baseline comparisons
- Imperative script-based deployment.
- Declarative reconciler without immutable closure discipline.
- Nix-style immutable deployments with minimal wave orchestration.

---

## Minimal reproducible structure for follow-up
- `program.md` defines objective, metric, constraints, and stop conditions.
- `notes.md` is append-only progress and decisions log.
- This `README.md` is the current report and model proposal.

## Reproduction notes
1. Enter the directory.
2. If using direnv + Nix:
   - `direnv allow`
   - tools from `flake.nix` become available.
3. Continue by adding:
   - a simulation script for rollout transitions,
   - scenario inputs,
   - measured MTTC outputs in an append-only `results.jsonl`.
