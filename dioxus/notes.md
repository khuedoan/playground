# Notes

## Objective

Replace the custom Dioxus component catalog and custom CSS with daisyUI.

Primary metric: custom CSS source lines. Unit: lines. Lower is better.

## Baseline

- `assets/styling/main.css`: 1,382 lines.
- `assets/dx-components-theme.css`: 70 lines.
- `src/components`: 1,904 Rust lines and 1,897 CSS lines.
- The app uses custom card, button, input, sidebar, tab, progress, switch, badge, and avatar components.

## Plan

- Follow the daisyUI Dioxus install guide.
- Build daisyUI with the Tailwind CSS CLI.
- Use daisyUI classes on native Dioxus elements.
- Use Tailwind utilities only for layout.
- Delete the custom component catalog and custom CSS.

## Result

- Replaced custom UI components with native Dioxus elements and daisyUI classes.
- Replaced custom layout selectors with Tailwind CSS utilities.
- Deleted all 1,904 Rust lines in `src/components`.
- Deleted 3,349 custom CSS lines across the app and component catalog.
- Reduced the authored CSS input to three daisyUI setup lines.
- Removed the `dioxus-primitives` dependency.
- Removed unused custom assets.
- Kept dynamic graph coordinates because daisyUI does not supply a graph layout.

## Verification

- `npm run build:css`: passed.
- `cargo check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo fmt --all -- --check`: passed.
- Browser route smoke test: passed for all routes.
- Desktop layout: passed at 1280 px width.
- Mobile layout and navigation drawer: passed at 390 px width.
- Browser console and page error checks: passed.

## Failed attempts

- The first local format check failed because `rustfmt` was not in the development shell.
- Added Nix packages for `rustfmt` and `clippy`.
