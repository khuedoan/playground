# Functional Deployment Model for Distributed Systems

## Research question
How can we extend the core ideas of the purely functional deployment model (as established for single-node systems) into a robust deployment model for distributed systems without losing reproducibility, rollbackability, and operational safety?

## Scope and assumptions
- The target environment is a fleet of nodes running services with inter-service dependencies.
- A central control plane computes rollout plans; node agents perform local activation.
- We optimize for correctness and operability first, then rollout speed.

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

### 4) Raft/consensus-inspired control-plane durability (influence only)
- Useful idea: control-plane decisions should be linearizable and replayable.
- Gap: consensus algorithms solve replicated log agreement, not deployment semantics.

Reference:
- Raft paper site: https://raft.github.io/

---

## Proposed model: Functional Distributed Deployment Model (FDDM)

### Design goals
1. Preserve Nix-like immutable closure semantics.
2. Add explicit distributed rollout semantics.
3. Keep recovery simple: rollback by epoch pointer, not ad hoc repair.
4. Maintain eventual convergence under partial failure.
5. Keep policy explicit and machine-checkable.

### Non-goals
- FDDM is not a consensus protocol replacement.
- FDDM does not assume zero downtime for all workloads.
- FDDM does not force one migration strategy; it codifies safe gates.

## Formalization

### Core objects
- **Spec** `S`: versioned desired system specification.
- **Inputs** `I`: pinned external inputs (artifact indexes, secrets references, topology facts).
- **Compiler** `C`: pure function `C(S, I) -> P`.
- **Plan** `P`: deterministic rollout graph with wave partitions and policies.
- **Epoch** `E_k`: immutable snapshot `E_k = materialize(P)`.
- **Node pointer** `ptr[n]`: active epoch for node `n`.
- **Observed state** `O`: health and telemetry facts.

### Determinism contract
`hash(E_k)` MUST be stable for identical `(S, I)`.

Operationally:
- identical inputs yield byte-identical epoch metadata,
- activation is pointer movement, not mutable edit-in-place.

### State machine
Global rollout state for epoch `k`:
- `Prepared(k)`
- `Activating(k, wave=i)`
- `Stable(k)`
- `Paused(k, reason)`
- `RolledBack(k->k-1, reason)`

Allowed transitions:
- `Prepared -> Activating(0)`
- `Activating(i) -> Activating(i+1)` when wave gate passes
- `Activating(last) -> Stable`
- `Activating(i) -> Paused` on policy breach
- `Paused -> Activating(i)` on manual resume
- `Paused/Activating -> RolledBack` on rollback decision

## Execution model

### Phase 1: Realization (side-effect controlled)
- Build/fetch all required closures.
- Validate artifact integrity and signatures.
- Preflight dependency checks (capacity, credentials, migration compatibility).
- Emit immutable epoch manifest and rollout graph.

### Phase 2: Activation (wave-based pointer switching)
For each wave `W_i`:
1. switch `ptr[n] := k` for all `n in W_i` (candidate activation),
2. evaluate node-local and service-level invariants,
3. compute wave verdict from policy thresholds,
4. advance, pause, or rollback.

Policy examples:
- max error budget consumed per wave,
- min ready replica ratio,
- bounded p95 latency regression.

### Phase 3: Stabilization (continuous reconciliation)
- Controller continually reconciles drift back to `E_k`.
- If global SLO or invariant violations persist past a grace window, auto-demote to `k-1` or require operator ack.

## Invariants
1. **Immutability:** epoch content never changes once published.
2. **Traceability:** every live node references an epoch ID and closure digest.
3. **Bounded divergence:** mixed-epoch operation is allowed only within policy-defined windows.
4. **Monotonic audit log:** all transitions append-only, with actor + reason.
5. **Deterministic rollback target:** rollback always points to a known prior epoch.
6. **Policy visibility:** gate decisions are explainable from stored evidence.

## Failure semantics
- **Node failure during wave:** hold/rollback only that wave unless policy escalates.
- **Partition:** continue only in components that still satisfy safety predicates; freeze elsewhere.
- **Schema migration mismatch:** gate next wave until compatibility checks pass; require dual-read/dual-write windows for breaking changes.
- **Control-plane restart:** replay append-only log to reconstruct exact rollout state.

## Why this is “functional” (not only declarative)
Declarative desired state alone is insufficient; FDDM additionally requires:
- content-addressed, immutable realization artifacts,
- pure compilation from spec to deployable graph,
- epoch-based activation semantics with rollback by pointer,
- reproducible gate decisions from persisted policy + evidence.

---

## Pseudocode (controller)

```text
propose(S, I):
  P  = C(S, I)
  Ek = materialize(P)
  append_log(PREPARED, Ek)

rollout(Ek):
  for wave in Ek.waves:
    append_log(WAVE_START, wave)
    activate_candidates(wave, Ek)
    verdict = evaluate(wave.policies, observe())
    append_log(WAVE_VERDICT, verdict)
    if verdict == PASS:
      commit_wave(wave, Ek)
    elif verdict == PAUSE:
      append_log(PAUSED, reason)
      return
    else:
      rollback_wave(wave, previous_epoch)
      append_log(ROLLED_BACK, reason)
      return
  append_log(STABLE, Ek)
```

## Worked scenario
- Fleet: 60 nodes, 3 waves of 20.
- Wave policy: proceed if ready ratio >= 0.95 and p95 latency regression <= 10%.
- Outcome:
  - wave 1 passes,
  - wave 2 fails latency gate,
  - controller rolls wave 2 back to `E_(k-1)`, wave 3 never starts,
  - system remains mixed-epoch only within bounded window, then converges back.

---

## Evaluation plan

### Primary metric
- **MTTC (Mean Time To Convergence)** in seconds; lower is better.

### Secondary metrics
- Rollback latency.
- Failed rollout rate.
- Drift incidents per week.
- Policy false-positive pause rate.

### Suggested benchmark scenarios
1. Normal rollout across 3 waves and 100 nodes.
2. Wave-2 induced failure in 15% of nodes.
3. Network partition splitting control plane from one zone.
4. Backward-compatible database migration, then incompatible migration attempt.
5. Controller crash/restart mid-wave with log replay.

### Baseline comparisons
- Imperative script-based deployment.
- Declarative reconciler without immutable closure discipline.
- Nix-style immutable deployments with minimal wave orchestration.

---

## Minimal reproducible structure for follow-up
- `program.md` defines objective, metric, constraints, and stop conditions.
- `notes.md` is append-only progress and decisions log.
- `results.tsv` is append-only run log for measured benchmarks.
- This `README.md` is the current report and model proposal.

## Reproduction notes
1. Enter the directory.
2. If using direnv + Nix:
   - `direnv allow`
   - tools from `flake.nix` become available.
3. Extend the experiment by adding:
   - a simulator for rollout state transitions,
   - scenario fixtures,
   - benchmark outputs appended to `results.tsv`.
