# Agent-owned Orb TLA+ model

`AgentOrbOwnership.tla` is a finite safety model for personal agent compute.
The permanent function `orbOwner` assigns exactly one owner to each orb and is
never changed. The same agent may own several orbs. Other agents can send and
receive messages, but message transitions cannot change resource, session, or
environment authority.

Shared environments are separate services. An effect needs an active owner
session and a current per-orb environment connection. Disconnecting the
environment advances its epoch and invalidates old submissions and retry
authority.

## Checked configurations

| Config | Purpose | Generated | Distinct | Depth | Result |
|---|---|---:|---:|---:|---|
| `OwnershipOneAgentManyOrbs.cfg` | One agent permanently owns two orbs | 11,781,089 | 2,311,872 | 29 | Pass |
| `OwnershipTwoAgents.cfg` | Two owners and data-only messages | 53,124 | 14,400 | 11 | Pass |
| `OwnershipCommand.cfg` | Owner-only fencing and version CAS | 128 | 104 | 9 | Pass |
| `OwnershipEffect.cfg` | Shared environment, retry, and escrow | 15,960 | 5,232 | 18 | Pass |
| `OwnershipUnsafe.cfg` | Commit without current validation | 51 before violation | 41 before violation | 6 | Expected failure |

The safe configurations check `TypeOK`, `OneOwnerPerOrb`, and the applicable
ownership, message, stale-commit, effect, retry, and budget invariants. The
unsafe trace is: start owner session, acquire resource, issue command, revoke
owner session, then incorrectly accept the stale command.

Run a configuration from this directory:

```sh
mkdir -p .tlc-tmp
LD_LIBRARY_PATH=/usr/lib/jvm/java-17-openjdk-amd64/lib \
java -Djava.io.tmpdir=.tlc-tmp -XX:+UseParallelGC -Xmx768m \
  -cp ../../tools/tla/tla2tools.jar tlc2.TLC -cleanup -workers 4 \
  -config OwnershipCommand.cfg AgentOrbOwnership.tla
```

These are exhaustive results only for the stated finite bounds. The model
proves safety, not liveness, authentication, isolation of real machines,
message confidentiality, or provider behavior.
