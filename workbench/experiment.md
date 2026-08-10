# Experiment

## Objective

Build a self-hosted, private-network coding workspace product where Phoenix owns durable product state and a Rust host agent provisions one NixOS MicroVM per thread.

## Files in scope

Only `workbench/` in `khuedoan/playground`.

## Constraints

- `microvm.nix` is the sole host backend.
- Each guest includes Wayland, Blender, code-server, Pi, and the language toolchains used by the agent.
- Production guests use local Qwen; the KVM end-to-end gate may substitute a deterministic guest-local API while keeping Pi and tool execution real.
- Provisioning remains idempotent across retries and host-agent restarts.
- No claim of an end-to-end VM boot without a KVM-capable NixOS host.

## Stop conditions

- Rust and Phoenix tests pass.
- Nix files format and evaluate on a Nix-capable runner.
- GitHub Actions boots and exercises a real KVM MicroVM and uploads its recorded browser session.
- No alternate host backend or backend selector remains.
- A draft PR records any validation that requires a NixOS/KVM host.
