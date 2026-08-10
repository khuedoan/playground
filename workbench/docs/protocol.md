# Backend protocol

The control plane reconciles desired state through the host agent:

```http
PUT /v1/workspaces/:workspace_id
Content-Type: application/json
```

```json
{
  "command_id": "019fe75c-da04-765a-a098-177e59959717",
  "workspace_id": "019fe75c-da04-765a-a098-177e59959716",
  "generation": 3,
  "desired_state": "running",
  "profile": {
    "vcpus": 4,
    "memory_mib": 8192,
    "disk_gib": 40,
    "gui": true
  }
}
```

Rules:

1. Repeating a `command_id` returns the recorded result without executing the backend again.
2. A generation lower than the recorded generation is rejected with `409 Conflict`.
3. Reusing a generation with a different command is rejected with `409 Conflict`.
4. A greater generation is applied and durably journaled before success is returned.

The response contains actual state and endpoints:

```json
{
  "workspace_id": "019fe75c-da04-765a-a098-177e59959716",
  "generation": 3,
  "actual_state": "running",
  "ip_address": "10.88.42.17",
  "desktop_url": "http://10.88.42.17:6080/vnc.html?autoconnect=1&resize=scale",
  "code_url": "http://10.88.42.17:3000",
  "agent_url": "http://10.88.42.17:7070"
}
```

Guest endpoints:

- `POST /v1/exec` starts a bounded shell command from a canonicalized directory inside `/workspace`. It is an execution API, not a filesystem sandbox; the MicroVM is the security boundary.
- `POST /v1/pi/sessions/:session/rpc` sends one JSON command to a persistent `pi --mode rpc` process and returns the correlated Pi response.

Pi uses strict LF-delimited JSONL. The guest reader splits only on `\n`; it does not use a Unicode-aware generic line protocol.

## MicroVM lifecycle mapping

For `running`, the host agent writes a generated flake, creates or updates the runner with the `microvm` command, starts `microvm@<workspace>.service`, and waits for the guest TCP endpoint. `stopped` stops that unit without deleting its volumes. `deleted` stops it and removes the exact MicroVM state, generated flake, and ephemeral credential-share directories.

The `VmBackend` trait is retained only as an internal test seam for the durable journal. The production binary constructs `MicrovmBackend` directly and exposes no backend selector.
