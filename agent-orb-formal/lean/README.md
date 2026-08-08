# Agent-owned Orb policy kernel

`Coord/Kernel.lean` is an executable transition kernel for one independently
serialized resource and escrow shard.

The state has one permanent `ownerSession.agentId`. No transition changes that
identity. An owner token must match it before the token can operate on the
orb's resource. `Token.restrict` can reduce scope but cannot change the agent.

Shared-environment effects require two current credentials:

- an owner token for the orb resource;
- a separate environment token for the connected shared service.

Submission and retry require both credentials to name the same owner. A
disconnect advances the environment epoch and makes the old environment token
stale. Broker receipts can still report the result of an already submitted
effect after owner-session revocation.

`AgentMessage` contains only sender, recipient, and payload hash. The modeled
message protocol has no orb, resource, or capability field.

The checked theorems cover permanent owner preservation, owner-only current
tokens, capability restriction, stale generation/owner/policy/fence rejection,
resource CAS, unique leases, shared-environment checks, budget conservation,
terminal escrow, explicit retry gating, and `Valid` preservation.

Standard toolchain:

```sh
lake build
```

The constrained workspace used this serial Lean 4 WASM frontend:

```sh
tools/lean-wasm -j1 formal/lean/Coord/Kernel.lean
```

## Trust boundary

Credential records represent inputs after gateway authentication. The proof
does not establish cryptographic authenticity, hypervisor isolation, database
durability, provider idempotency, or that every production request goes
through this kernel. Shared-environment services must independently enforce
their capability. The delivery cursor is not a proof of causal consistency.
