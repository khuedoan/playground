# Workbench

Workbench asks whether a small Phoenix and Rust product can provide ChatGPT Work-style coding threads entirely inside a private network, with a hardware-virtualized Linux environment per thread.

The result is a `microvm.nix`-only prototype. Phoenix stores product intent and durable Oban jobs. A Rust host agent turns that intent into generated MicroVM flakes and reconciles `microvm@…` systemd units. A Rust guest agent keeps one Pi JSONL-RPC process per conversation.

## Result

The production backend count is one: [`microvm.nix`](https://github.com/astro/microvm.nix). Pi receives a complete Linux developer environment directly from the declarative NixOS guest configuration.

Each workspace includes:

- a NixOS MicroVM running through Cloud Hypervisor and KVM;
- a persistent `/workspace` volume;
- a private TAP address on `10.88.0.0/16` with host NAT;
- headless Sway/Wayland, wayvnc, noVNC, Blender, and code-server;
- the Nixpkgs `pi-coding-agent` package and a persistent Rust Pi RPC bridge;
- Rust, Node.js, Python, Git, and build tools inside the guest.

The originating Work sandbox could compile and test the application code but could not boot a MicroVM because it exposes no KVM, `/proc`, cgroups, or Nix daemon. A real NixOS/KVM boot remains an explicit validation gate rather than a simulated result.

## Architecture

```mermaid
flowchart TD
    B["Browser / LiveView"] --> P["Phoenix control plane"]
    P --> DB[("PostgreSQL + Oban")]
    P --> H["Rust host agent"]
    H --> M["microvm command + systemd"]
    M --> V["NixOS MicroVM"]
    V --> G["Rust guest agent + Pi RPC"]
    V --> UI["Sway + noVNC + code-server"]
```

Phoenix persists `{command_id, workspace_id, generation, desired_state}` before enqueueing reconciliation. The host journal makes retries idempotent and rejects stale generations. For a new workspace, the host agent writes a small flake under `/var/lib/workbench/specs`, runs `microvm -f … -c …`, starts `microvm@<workspace>.service`, and waits for the guest agent on port 7070.

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
          services.workbench = {
            enable = true;
            environmentFile = "/run/secrets/workbench-model.env";
          };
        }
      ];
    };
  };
}
```

The environment file stays outside the Nix store and can contain:

```bash
OPENAI_API_KEY=...
PI_PROVIDER=openai
PI_MODEL=...
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

## Tests

```bash
nix develop
make check
```

`nix flake check` builds both Rust agents. `cargo test` covers command idempotency, stale generations, concurrent retries, generated workspace flakes, and MicroVM lifecycle commands. Phoenix tests cover durable jobs, audit events, reconciliation, and LiveView behavior.

GitHub Actions also runs `scripts/e2e.sh` on a KVM-enabled Linux runner. It provisions the production host agent and official MicroVM runner, boots a real Cloud Hypervisor guest, creates a workspace through Phoenix, asks the real Pi process to modify and verify a file using GitHub Models, checks code-server and the Wayland/noVNC/Blender services, restarts the MicroVM, and verifies the workspace file persisted. The job uploads the browser video and host/guest diagnostics as `workbench-real-microvm-<run-id>`.

## Real demo recording

After the NixOS host and Phoenix are running:

```bash
./scripts/demo.sh
```

The script refuses to run without `/dev/kvm`, an active host agent, and a reachable Phoenix UI. Playwright then creates a real MicroVM through the UI, waits for a real Pi response, and records `demo/artifacts/workbench-demo.webm`. It has no mock or simulated-success path. When `ffmpeg` is available it also emits `demo/artifacts/workbench-demo.mp4`.

## Security boundary

The MicroVM is the workspace isolation boundary. This is still a prototype, not a finished hostile multi-tenant service. Before production, add user authentication and authorization, signed host/guest requests, per-workspace firewall rules, collision-free IP allocation, secret-brokered short-lived model credentials, image/flake review, resource admission control, and escape/cross-tenant release tests.

Private networking also does not make hosted inference private. Configure Pi for an internal OpenAI-compatible, Ollama, vLLM, or LM Studio endpoint when prompts and files must not leave the network.

See [docs/protocol.md](docs/protocol.md), [docs/security.md](docs/security.md), [notes.md](notes.md), and [experiment.md](experiment.md).
