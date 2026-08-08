# Agent-Orb Fabric TLA+ model

`AgentOrbFabric.tla` is a finite safety model of the control protocol. It
models agent/orb presences, supervisor-issued roles, revocation epochs, orb
failure and generation recovery, resource ownership/fencing/version-CAS,
stamped commands, broker effects, reconciled retries, and per-effect budget
escrow.

It does **not** model or verify checkpoint consistency, causal delivery,
liveness, the supervisor's authentication, storage isolation, or an
implementation refinement. The TLC results below are exhaustive only for the
listed finite configurations.

## Checked configurations

| Config | Purpose | Generated | Distinct | Depth | Result |
|---|---|---:|---:|---:|---|
| `AgentOrbFabric.cfg` | Combined 1-agent/1-orb interactions | 10,446 | 4,366 | 17 | Pass |
| `AgentOrbFabricPresence.cfg` | Dynamic 2-agent x 2-orb presence graph | 1,047,329 | 135,424 | 15 | Pass |
| `AgentOrbFabricCommand.cfg` | Command generation/epoch/fence/CAS | 257 | 167 | 10 | Pass |
| `AgentOrbFabricEffect.cfg` | Effect retry/idempotency/escrow | 2,238 | 857 | 13 | Pass |

The checked invariants are `TypeOK`, `OwnerAuthorized`, `NoStaleCommit`,
`BudgetConserved`, `TerminalEscrowSettled`,
`RetryOnlyAfterReconciliation`, and `EffectWellFormed`.

Run from this directory with the bundled official TLA+ tools:

```sh
mkdir -p .tlc-tmp
LD_LIBRARY_PATH=/usr/lib/jvm/java-17-openjdk-amd64/lib \
java -Djava.io.tmpdir=.tlc-tmp \
  -cp ../../tools/tla/tla2tools.jar tlc2.TLC \
  -config AgentOrbFabric.cfg -workers 2 AgentOrbFabric.tla
```

Safety caveat: after an `Unknown` effect is reconciled as not having occurred,
the checked model keeps its retry bound to the original generation, presence,
and policy stamps. Revocation or recovery can therefore strand that effect's
escrow. A production protocol needs a supervisor-linearized effect rebind to a
new authorized presence, or a terminal reconciliation/cancellation operation
that refunds the escrow. Progress of either operation is not proved here.

The earlier `AgentOrb.tla`/`AgentOrbUnsafe.cfg` intentionally demonstrates a
stale command accepted when commit-time validation is disabled. Its safe
configuration is a narrower scalar-orb model and must not be used to claim
verification of the full fabric.
