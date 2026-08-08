# Agent-Orb Fabric formal verification

This bundle contains the executable TLA+ safety model and Lean 4 policy kernel
used to revise the architecture.

## Verified results

### TLA+ / TLC

`tla/AgentOrbFabric.tla` models finite agents, orbs, agent-orb presences,
supervisor admission and revocation, orb failure/recovery, resource actors,
commit-time fencing/CAS, broker effects, retries, and terminal budget escrow.

Official TLC exhaustively completed the supplied finite configurations:

| Configuration | Distinct states | Depth | Result |
|---|---:|---:|---|
| Combined interactions | 4,366 | 17 | Pass |
| 2 agents x 2 orbs presence topology | 135,424 | 15 | Pass |
| Command fencing and CAS | 167 | 10 | Pass |
| Effect retry and escrow | 857 | 13 | Pass |

The deliberately unsafe scalar model produces a six-state stale-commit
counterexample after revocation. Its safe variant explored 4,901,580 distinct
states with no invariant violation, but that result is intentionally narrower
than the fabric model.

### Lean 4

`lean/Coord/Kernel.lean` is an executable pure transition kernel for one
independently serialized resource/escrow actor shard.

Frozen source SHA-256:

`c54702d6ee81c5677a1f5c336249bfa947dbeabaa1bdb043e55e9f6284667c4a`

Two independent serial Lean checks exited 0 with `hasErrors=false` and
`env?=true`. Source lint found no `sorry`, `admit`, `axiom`, `unsafe`, or
`partial` declarations.

The theorems cover same-presence restriction, commit-time currentness,
generation/revocation/policy/fence rejection, exclusive ownership, resource
CAS, bounded monotone delivery cursors, terminal escrow conservation and
duplicate rejection, explicit retry gating, broker/admin authority, and
preservation of `Valid` across every successful transition.

## Architecture forced by the formal work

1. An orb is a supervisor over independent resource actors, not ambient shared
   mutable memory.
2. A presence is supervisor-minted authority scoped to one agent and orb.
3. Every mutation atomically rechecks orb generation, presence revocation
   epoch, policy epoch, resource ownership/fence, and resource version.
4. Orb failure invalidates all presences and leases; recovery creates a higher
   generation.
5. Cross-agent delegation mints a fresh target presence; bearer forwarding is
   not supported.
6. External effects use a broker, stable idempotency keys, fenced receipts,
   explicit reconciliation, and one terminal escrow per effect.
7. An `Unknown` effect may accept a late matching confirmation. Retry or
   reassignment is supervisor-mediated.
8. Multi-resource work is a saga over versioned actors. A checkpoint is only an
   immutable version manifest unless resource adapters explicitly quiesce.

## Proof boundary

The verification assumes authenticated supervisor/broker/token inputs and
linearizable durable CAS at each actor shard. It does not prove cryptography,
hypervisor isolation, storage implementations, provider idempotency, full
causal consistency, checkpoint quiescence, liveness, or refinement of a
production implementation.

See `tla/README.md` and `lean/README.md` for exact model bounds and commands.
