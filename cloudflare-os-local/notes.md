# Notes

## 2026-08-06 — baseline and implementation

- Baseline: the playground had no Cloudflare OS experiment or local runner.
- Confirmed upstream's `pnpm run-local` starts Wrangler, which uses `workerd`
  underneath and stores local state in `.wrangler`.
- Pinned Cloudflare OS commit `aedcda8b3066ff666f57ae28ecef7341d6c2dee7`.
- Verified its source archive with SHA-256
  `d0dff6a820f6e19c42eba02d937c6330de3a2c6a0de9bcb3a294262376360c93`.
- Added a minimal patch that binds Wrangler to `0.0.0.0`, allowing Docker's
  published port to reach it. No upstream source is vendored.
- Chose Compose over a native workerd configuration because Cloudflare OS's
  upstream self-hosted workerd documentation is explicitly not yet available;
  its supported local path already runs workerd through Wrangler.
- Applied the patch to a clean copy, installed the frozen dependency graph,
  built the typed-storage and frontend packages, and started the complete local
  stack with Node 24 and pnpm 11.17.0. The readiness check returned HTTP 200:
  **1/1 successful requests**, meeting the primary metric.
- Docker and Nix were not installed in the execution environment, so their
  wrappers were checked statically rather than invoked here.

## 2026-08-06 — Compose startup fix

- Baseline: Compose exited with `spawn ps ENOENT` when Wrangler restarted a
  child build after generated files changed.
- Cause: the Debian Slim image does not include the `ps` command.
- Added the Debian `procps` package, which supplies `ps`.
- Rebuilt the image, started the service, and received HTTP 200 from `/`.
  Wrangler completed the watched build restart without the prior error.
