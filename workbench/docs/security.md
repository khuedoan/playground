# Security notes

## Isolation boundary

Each workspace runs as a NixOS MicroVM with its own kernel, memory allocation, TAP interface, persistent workspace volume, Wayland compositor, and Pi process. Cloud Hypervisor and KVM provide a substantially stronger boundary than a privileged shared-kernel sandbox.

The host agent runs as root because it creates MicroVM definitions and controls `microvm@…` systemd services. Keep its HTTP listener on loopback and allow only Phoenix to call it. The guest agent, code-server, and noVNC are intentionally unauthenticated on the private workspace subnet in this prototype; do not route that subnet to untrusted clients.

## Credentials

The NixOS module accepts a host-side environment file outside the Nix store. The host agent copies only supported model variables into a root-owned file under `/run/workbench/credentials/<workspace>` and shares that directory read-only in spirit through virtiofs at boot. The directory is removed when the workspace is deleted and disappears on host reboot.

This avoids embedding secrets in generated flakes, Nix derivations, command arguments, or the host command journal. It is not a complete secret-broker design: use scoped, short-lived credentials and rotate them independently of VM state.

Private-network access does not automatically make inference private. Hosted model providers receive whatever Pi sends as context. For sensitive data, configure Pi with an internal Ollama, vLLM, LM Studio, or OpenAI-compatible endpoint and block direct internet egress from workspace TAP interfaces.

## Before hostile multi-tenancy

- Authenticate Phoenix users and authorize every workspace query.
- Sign host and guest requests and bind them to workspace identities.
- Allocate IPs with a persistent collision-free allocator instead of the current deterministic prototype mapping.
- Add per-workspace firewall and outbound network policy.
- Enforce CPU, memory, disk, VM-count, and build admission limits.
- Pin and review flake inputs, use binary-cache signatures, and define an update policy.
- Encrypt PostgreSQL and MicroVM volumes and define deletion/backup semantics.
- Move model access behind a secret broker or internal inference gateway.
- Run MicroVM escape, cross-workspace reachability, and stale-volume tests as release gates.
