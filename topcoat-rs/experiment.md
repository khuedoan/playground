# daisyUI migration

## Objective

Replace the custom UI system with daisyUI defaults.

## Metric

- Name: Local UI code lines
- Unit: lines
- Direction: Lower is better

## Scope

- `assets/`
- `build.rs`
- `Cargo.toml`
- `flake.nix`
- `Makefile`
- `package.json`
- `src/components*`
- `src/layout.rs`
- `src/main.rs`
- `src/pages.rs`
- `tailwind.css`

## Constraints

- Keep current routes and behavior.
- Use Nix for tools.
- Use the default daisyUI theme.
- Do not edit other experiments.

## Stop conditions

- The application builds.
- Rust tests pass.
- daisyUI generates the application CSS.
- Core pages work at desktop and mobile widths.
- No custom CSS theme or local component style sheet remains.
