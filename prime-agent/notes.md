# Notes

## Objective

Provide Prime Agent through `nix develop`. Do not execute the upstream shell installer.

Primary metric: `prime-agent --version` availability in the development shell, measured as pass or fail. Pass is better.

## Baseline

- `flake.nix` was empty.
- The `prime-agent` command was not available from this experiment.

## Work log

- Inspected the public source repository at commit `c22549a37b73cc603c6f0d202517cb0ca856c7d3`.
- Tried `buildNpmPackage` with the upstream npm v3 lock. Nix could not fetch entries that omitted registry URLs and integrity values.
- Kept the upstream package versions and added the missing npm registry metadata in `package-lock.json` for the Nix fixed-output fetcher.
- Skipped the network-based model catalog generator during the build. The build uses the catalog that is tracked in the source repository.
- Initially added an immutable Nix Python kernel and default packages.
- Review found that setting `PRIME_AGENT_KERNEL_PYTHON` bypassed upstream bootstrap and disabled the separately packaged Python-backed skills.
- Removed the kernel override. The wrapper now provides `uv` and Python 3.11 on `PATH`, while Prime Agent creates and manages its normal writable kernel environment under `~/.prime`.
- Built the package and tested `nix develop -c prime-agent --version`.

## Result

Primary metric: pass. The command returned version `0.7.0`.
