# Netamos Mockup

This Dioxus 0.7 app is a static routed mockup for Netamos, a network-aware
platform as a service control plane.

## What Is Included

- Tenant overview for projects, environments, components, private links, and spaces.
- Project topology graph inferred from private-link intent and observed traffic.
- Project inventory and project detail views.
- Component topology graph inside each project detail view, inferred from network
  telemetry and Vault config references.
- Private link handshake view with source request, target allow, and space checks.
- Space inventory for shared hosted spaces and tenant-owned enterprise spaces.
- Prebuilt Dioxus catalog components for sidebar layout, cards, buttons, badges, inputs,
  tabs, progress bars, switches, labels, and avatar UI.
- Static mock data only. Controls are present for product realism, but no backend
  workflow is connected.

## Run

Start the web dev server:

```sh
make dev
```

The Dioxus CLI prints the local URL after the web bundle builds.

## Verify

Run the Rust check:

```sh
cargo check
```

The generated component catalog currently emits dead-code warnings for variants that
this mockup does not use.
