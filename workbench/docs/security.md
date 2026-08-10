# Security notes

## Isolation boundary

Each workspace runs as a NixOS MicroVM with its own kernel, memory allocation, TAP interface, persistent workspace volume, Wayland compositor, and Pi process. Cloud Hypervisor and KVM provide a substantially stronger boundary than a privileged shared-kernel sandbox.

The host agent runs as root because it creates MicroVM definitions and controls `microvm@…` systemd services. Keep its HTTP listener on loopback and allow only Phoenix to call it. The guest agent, code-server, and noVNC are intentionally unauthenticated on the private workspace subnet in this prototype; do not route that subnet to untrusted clients.

## Model privacy

Production guests run Pi against a llama.cpp server bound to guest loopback. The pinned Qwen2.5 Coder GGUF is part of the default declarative guest closure; prompts, tool schemas, and model output stay inside the MicroVM.

The GitHub Actions end-to-end configuration explicitly enables a deterministic OpenAI-compatible mock on the same guest-loopback address. This test-only service needs no credential and replaces only model generation; Pi, tool execution, and the MicroVM boundary remain real.

Private-network access would not automatically make hosted inference private. If the local model is replaced, use an internal inference endpoint and block direct internet egress from workspace TAP interfaces. Do not put long-lived provider secrets in generated flakes or the Nix store.

## Before hostile multi-tenancy

- Authenticate Phoenix users and authorize every workspace query.
- Sign host and guest requests and bind them to workspace identities.
- Allocate IPs with a persistent collision-free allocator instead of the current deterministic prototype mapping.
- Add per-workspace firewall and outbound network policy.
- Enforce CPU, memory, disk, VM-count, and build admission limits.
- Pin and review flake inputs, use binary-cache signatures, and define an update policy.
- Encrypt PostgreSQL and MicroVM volumes and define deletion/backup semantics.
- Keep model access local or place it behind an internal inference gateway and secret broker.
- Run MicroVM escape, cross-workspace reachability, and stale-volume tests as release gates.
