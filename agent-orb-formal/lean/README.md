# Agent–Orb policy kernel

`Coord/Kernel.lean` is an executable transition kernel for one independently
serialized resource/escrow actor shard. It deliberately does not introduce a
system-global lock.

The checked theorems cover:

- same-presence capability restriction without privilege increase;
- current generation, policy epoch, revocation epoch, expiry, resource fence,
  exclusive ownership, and resource-version admission;
- a structurally unique exclusive owner;
- bounded monotone event-delivery cursors;
- one-operation budget escrow, terminal settlement/refund, conservation, and
  rejection of a duplicated terminal receipt;
- explicit retry gating with a stable provider idempotency key;
- broker reconciliation after agent revocation; and
- preservation of the `Valid` state predicate by every successful transition.

Standard toolchain:

```sh
lake build
```

The constrained workspace used a serial Lean 4 WASM frontend instead:

```sh
tools/lean-wasm -j1 formal/lean/Coord/Kernel.lean
```

## Trust boundary

Credential records in the model represent tokens or receipts after gateway
authentication. The proof does not establish cryptographic authenticity,
hypervisor isolation, database durability, provider idempotency, or that all
production commands pass through this kernel. Cross-agent delegation is not a
bearer-token operation: a supervisor must mint a new target-presence token and
escrow. The delivery cursor proof is not a proof of full causal consistency.
