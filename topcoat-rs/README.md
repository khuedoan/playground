# daisyUI migration

## Question

Can default daisyUI replace the local Topcoat UI layer?

## Setup

The application keeps the Topcoat server and routes.

Tailwind CSS uses the official daisyUI install steps for Dioxus.

`tailwind.css` has only the source, Tailwind, and daisyUI lines.

The application uses the default light theme.

The views use default daisyUI components directly.

Tailwind utilities only control layout and responsive placement.

## Result

The local UI layer decreased from 1,086 lines to 3 lines.

The custom theme, component adapters, and component style files are gone.

The Rust tests pass.

Clippy passes with warnings set as errors.

Desktop and mobile browser checks pass.

## Reproduce

Enter the Nix shell and install the locked Node packages.

```sh
nix develop
npm ci
```

Run all tests.

```sh
make test
cargo clippy --all-targets -- -D warnings
```

Run the CSS watcher and the application server.

```sh
make dev
```
