# Agent-owned Orb formal verification

This bundle contains the executable TLA+ safety model and Lean 4 policy kernel
for personal agent compute.

## Final ownership model

1. One agent can permanently own many orbs.
2. Each orb has exactly one permanent agent owner.
3. Only that owner can acquire or mutate the orb's resources.
4. Agents communicate with data-only messages. Messages do not carry orb
   authority.
5. An orb can connect to a shared environment with a separate, revocable
   environment capability.
6. Every resource mutation rechecks the orb generation, owner-session epoch,
   policy epoch, resource owner/fence, and expected resource version at commit.

This is an actor-style implementation, but it is not the classic model of
freely shared actors. Each orb and resource actor has a strict authority
boundary.

## Verified results

### TLA+ / TLC

`tla/AgentOrbOwnership.tla` models permanent orb ownership, owner sessions,
agent messages, shared-environment connections, resource fencing and CAS,
failure/recovery, broker effects, retries, and terminal budget escrow.

Official TLC exhaustively completed these finite configurations:

| Configuration | Distinct states | Depth | Result |
|---|---:|---:|---|
| One agent owns two orbs | 2,311,872 | 29 | Pass |
| Two agents exchange messages | 14,400 | 11 | Pass |
| Owner command fencing and CAS | 104 | 9 | Pass |
| Shared-environment effect and escrow | 5,232 | 18 | Pass |

The unsafe configuration produces a six-state stale-commit counterexample
when commit-time validation is disabled.

### Lean 4

`lean/Coord/Kernel.lean` is an executable pure transition kernel for one
independently serialized resource/escrow actor shard. A serial Lean check exits
0 with `hasErrors=false` and `env?=true`. Source lint contains no `sorry`,
`admit`, `axiom`, `unsafe`, or `partial` declarations.

The theorems cover permanent owner preservation, owner-only current tokens,
same-owner restriction without privilege increase, commit-time epoch/fence/CAS
checks, shared-environment admission, disconnected-environment rejection,
terminal escrow conservation and duplicate rejection, explicit retry gating,
and preservation of `Valid` across every successful transition.

Frozen source SHA-256:
`08be3660f956cfdd339786c2c3459c575c6e65fc89e04ed431db34c436009718`

## Proof boundary

The verification assumes authenticated supervisor, broker, token, and receipt
inputs plus linearizable durable CAS at each actor shard. It does not prove
cryptography, hypervisor isolation, storage durability, provider idempotency,
message confidentiality, liveness, or refinement of a production
implementation. Shared environments must enforce their own capability at the
service boundary.

See `tla/README.md` and `lean/README.md` for exact commands and bounds.
