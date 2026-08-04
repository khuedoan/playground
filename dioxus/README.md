# Netamos Mockup

This Dioxus 0.7 app is a static Netamos control-plane mockup.
It uses [daisyUI for Dioxus](https://daisyui.com/docs/install/dioxus/).
It has no custom CSS selectors or custom UI component library.

The app shows projects, private links, compute spaces, and topology data.
The data is static.
The visible actions do not connect to a backend.

## Setup

Enter the Nix development shell:

```sh
direnv allow
```

Install the locked JavaScript packages:

```sh
npm ci
```

## Run the app

Run the Tailwind CSS watcher and the Dioxus server:

```sh
make dev
```

The Dioxus CLI shows the local URL.

## Build the CSS

Build the minified daisyUI file:

```sh
make build-css
```

The command reads `tailwind.css`.
It writes `assets/main.css`.

## Verify the app

Build the CSS and check the Rust code:

```sh
make check
```

Run all Rust lint checks:

```sh
cargo clippy --all-targets --all-features -- -D warnings
```

## Result

The migration removed 1,904 lines of custom component Rust code.
It also removed 3,349 lines of custom CSS.
The only authored CSS source is the three-line daisyUI setup in `tailwind.css`.
