# Notes

## 2026-08-04

- Objective: Replace the local Topcoat UI theme and components with daisyUI defaults.
- Primary metric: Local UI code lines. Unit: lines. Lower is better.
- Baseline: 1,086 lines in `styles.css` and `src/components/*.rs`.
- Baseline checks: 21 Rust tests pass.
- Baseline check: Clippy fails on three existing warnings in `src/pages.rs`.
- Decision: Keep the Topcoat server and renderer. Use the official daisyUI Dioxus CSS install method.
- Decision: Use the default daisyUI theme. Do not add a custom theme.
- Result: `tailwind.css` has only the three official install lines.
- Result: Deleted the custom theme and all local UI component adapters.
- Result: The views now use default daisyUI components directly.
- Result: Replaced the custom application shell with daisyUI drawer, navbar, menu, and dropdown components.
- Result: Replaced custom status, tab, statistic, divider, and checkbox styles with daisyUI components.
- Result: The local UI layer decreased from 1,086 lines to 3 lines.
- Result: 21 Rust tests pass.
- Result: Clippy passes with warnings set as errors.
- Result: Desktop, mobile, menu, and form browser checks pass.
