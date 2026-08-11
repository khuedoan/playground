# Workbench

Workbench asks whether a small Phoenix and Rust product can provide ChatGPT Work-style coding threads entirely inside a private network, with a hardware-virtualized Linux environment per thread and an interactive start time below one second.

The result is a `microvm.nix`-only prototype. A Rust host agent boots a fixed pool of generic MicroVMs before exposing its API, so Phoenix can lease an already-running slot when a thread is created. Phoenix stores thread state, messages, and durable Oban jobs. A Rust guest agent keeps one Pi JSONL-RPC process per conversation.

## Result

The production backend count is one: [`microvm.nix`](https://github.com/astro/microvm.nix). Pi receives a complete Linux developer environment directly from the declarative NixOS guest configuration.

Each workspace includes:

- a NixOS MicroVM running through Cloud Hypervisor and KVM;
- a persistent `/workspace` volume;
- a private TAP address on `10.88.0.0/16` with host NAT;
- headless Sway/Wayland, wayvnc, noVNC, Blender, and code-server;
- the Nixpkgs `pi-coding-agent` package and a persistent Rust Pi RPC bridge;
- Rust, Node.js, Python, Git, and build tools inside the guest.

The default host pool contains three guests, each with 4 vCPUs, 8 GiB of memory, and a 40 GiB persistent disk. Threads are isolated from one another and can run concurrently up to the pool capacity. Stopping a thread retains its slot and disk for fast resumption; deleting it wipes and reheats the slot before releasing it.

The originating Work sandbox could compile and test the application code but could not boot a MicroVM because it exposes no KVM, `/proc`, cgroups, or Nix daemon. A real NixOS/KVM boot remains an explicit validation gate rather than a simulated result.

## Architecture

```mermaid
flowchart TD
    B["Browser / LiveView"] --> P["Phoenix control plane"]
    P --> DB[("PostgreSQL + Oban")]
    P --> H["Rust host agent"]
    H --> W["Warm MicroVM pool"]
    W --> V1["Thread slot 1"]
    W --> V2["Thread slot 2+"]
    V1 --> G["Guest agent + Pi RPC"]
    V1 --> UI["Sway + noVNC + code-server"]
```

Phoenix persists `{command_id, workspace_id, generation, desired_state}` before enqueueing reconciliation. The host journal makes retries idempotent and rejects stale generations. At host startup, the agent builds and starts `workbench-pool-00`, `workbench-pool-01`, and subsequent fixed slots, then waits for every guest agent to become healthy before listening on port 9090. A new thread atomically leases a healthy unassigned slot, receives its fixed private address, and can immediately use the guest agent on port 7070. Pool assignments survive host-agent restarts.

## NixOS host setup

Requirements:

- x86-64 NixOS with `/dev/kvm` and systemd;
- enough RAM and disk for the selected workspace profile;
- PostgreSQL for Phoenix;
- a route from product users to the workspace subnet, or an authenticated reverse proxy for workspace endpoints.

Add the module to the host flake:

```nix
{
  inputs.workbench.url = "github:khuedoan/playground?dir=workbench";
  inputs.workbench.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { self, nixpkgs, workbench, ... }: {
    nixosConfigurations.agent-host = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        workbench.nixosModules.host
        {
          services.workbench.enable = true;
        }
      ];
    };
  };
}
```

Apply the host configuration and verify the private API:

```bash
sudo nixos-rebuild switch --flake .#agent-host
curl http://127.0.0.1:9090/healthz
```

The response identifies `microvm.nix` as the backend. The API is deliberately loopback-only by default; Phoenix should be the only caller.

## Phoenix control plane

Development tools are defined by the experiment-local flake:

```bash
nix develop
make setup

cd apps/control_plane
export DATABASE_URL=ecto://postgres:postgres@127.0.0.1/workbench
export SECRET_KEY_BASE="$(mix phx.gen.secret)"
export HOST_AGENT_URL=http://127.0.0.1:9090
MIX_ENV=prod mix ecto.setup
MIX_ENV=prod mix phx.server
```

Open <http://127.0.0.1:4000>, create a workspace, and wait for the durable reconciliation job to report `running`.

The UI provides a persistent thread list, focused conversation, and live workspace inspector in a dark three-pane layout inspired by ChatGPT Work, Codex, and Amp. Starting more than one thread leases separate guests; selecting a thread switches its message history, desktop, code-server link, runtime status, slot, and IP without stopping the others.

## Tests

```bash
nix develop
make check
```

`nix flake check` builds both Rust agents. `cargo test` covers command idempotency, stale generations, concurrent retries, pool assignment affinity and exhaustion, and MicroVM lifecycle commands. Phoenix tests cover durable jobs, persisted messages, audit events, reconciliation, and LiveView behavior.

GitHub Actions also runs `scripts/e2e.sh` on a KVM-enabled Linux runner. It provisions the production host agent and official MicroVM runner, boots two real Cloud Hypervisor guests into a warm pool, and creates two threads through Phoenix. For this test only, the guest configuration replaces Qwen and llama.cpp with a deterministic guest-local OpenAI-compatible API. The real Pi process consumes that API, executes its real `bash` tool to modify and verify a file, and returns the final response. The test asserts that both threads become usable in under one second, use distinct slots and IPs, cannot see each other's files, and keep running concurrently. It also checks code-server and the Wayland/noVNC/Blender services, restarts one MicroVM, and verifies its workspace file persisted. This keeps the isolation and agent integration coverage while removing model download and inference from the critical path. The job uploads the browser video and host/guest diagnostics as `workbench-real-microvm-<run-id>`.

## Real demo recording

After the NixOS host and Phoenix are running:

```bash
./scripts/demo.sh
```

The script refuses to run without `/dev/kvm`, an active host agent, and a reachable Phoenix UI. Playwright then creates two threads through the UI, keeps both visible, selects the primary thread, waits for a real Pi response, verifies the dark Wayland framebuffer, and records `demo/artifacts/workbench-demo.webm`. The CI configuration mocks only the LLM API response; it does not bypass the MicroVMs, Pi RPC, tool execution, multi-thread UI, isolation, or persistence paths. A normal host-agent deployment continues to use the local Qwen model. When `ffmpeg` is available the script also emits `demo/artifacts/workbench-demo.mp4`.

## Security boundary

The MicroVM is the workspace isolation boundary. This is still a prototype, not a finished hostile multi-tenant service. Before production, add user authentication and authorization, signed host/guest requests, per-workspace firewall rules, collision-free IP allocation, secret-brokered short-lived model credentials, image/flake review, resource admission control, and escape/cross-tenant release tests.

The prototype defaults to a small local GGUF coding model, so the demo prompt and file content never leave the MicroVM. A production deployment can replace it with a stronger model served by a private OpenAI-compatible inference endpoint.

See [docs/protocol.md](docs/protocol.md), [docs/security.md](docs/security.md), [notes.md](notes.md), and [experiment.md](experiment.md).
