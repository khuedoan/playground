# Experiment

## Objective

Build a self-hosted, private-network coding workspace product where Phoenix owns durable product state and a Rust host agent leases one prewarmed NixOS MicroVM per thread. A user should be able to start multiple isolated threads, with each thread becoming interactive in under one second once the host is ready.

## Files in scope

Only `workbench/` in `khuedoan/playground`.

## Constraints

- `microvm.nix` is the sole host backend.
- Each guest includes Wayland, Blender, code-server, Pi, and the language toolchains used by the agent.
- Production guests use local Qwen; the KVM end-to-end gate may substitute a deterministic guest-local API while keeping Pi and tool execution real.
- The host prewarms a fixed MicroVM pool before serving requests; thread creation must not build or cold-boot a guest.
- Concurrent threads use distinct pool slots, IP addresses, persistent disks, and message histories.
- Provisioning remains idempotent across retries and host-agent restarts.
- No claim of an end-to-end VM boot without a KVM-capable NixOS host.

## Stop conditions

- Rust and Phoenix tests pass.
- Nix files format and evaluate on a Nix-capable runner.
- GitHub Actions boots and exercises a real KVM MicroVM and uploads its recorded browser session.
- The KVM gate runs at least two threads concurrently and reports every warm-pool lease below 1,000 ms.
- The gate proves cross-thread file isolation and persistence after restarting one assigned MicroVM.
- No alternate host backend or backend selector remains.
- A draft PR records any validation that requires a NixOS/KVM host.
