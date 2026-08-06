# Cloudflare OS local PoC

This experiment asks: **can the open-source Cloudflare OS run locally through
Cloudflare's `workerd` runtime with one reproducible command?**

The answer is a Docker Compose wrapper around Cloudflare OS's supported
`pnpm run-local` path. Upstream launches Wrangler, and Wrangler runs the Workers
stack on `workerd`. The source is pinned to a commit and its archive checksum is
verified during the image build.

## Result

The primary metric is **local HTTP readiness**, measured as successful HTTP
responses at `/` (count, higher is better; target 1/1). The service is exposed
at <http://localhost:8787> and Wrangler data is retained in a named volume.

This is a development PoC, not a production deployment. Upstream warns that
`workerd` alone is not a hardened sandbox for untrusted code.

## Prerequisites

- Docker with the Compose plugin
- Nix with flakes enabled (recommended for the exact host-side tools)

Enter the pinned tool shell:

```sh
nix develop
```

If Docker Compose is already installed, Nix is optional for running the PoC.

## Run

```sh
make up
```

Wait for the startup message, then open <http://localhost:8787>. To use another
host port, run `PORT=9876 make up` and visit `http://localhost:9876`.

In a second terminal, verify readiness:

```sh
curl --fail --retry 30 --retry-delay 2 http://localhost:8787/
```

Stop the service while retaining data with `make down`. Remove the service and
all persisted local data with `make clean`.

## Reproducibility and architecture

- `flake.lock` pins the Nix packages supplying Docker Compose, Make, and curl.
- `compose.yaml` pins the Cloudflare OS Git commit and expected archive digest.
- `Dockerfile` pins Node and pnpm, verifies/extracts upstream, and installs the
  frozen pnpm lockfile. It also installs `procps`, which Wrangler needs to stop
  child build processes. Upstream's runner builds the required packages.
- `bind-all-interfaces.patch` is the only upstream delta; it lets the container
  publish Wrangler's server without copying the external repository here.
- The `.wrangler` directory lives in the `cloudflare-os-data` named volume.

To update upstream, change both build arguments in `compose.yaml`, update the
matching Dockerfile defaults, and append the new commit and archive digest to
`notes.md`.

## Useful commands

```sh
make check  # validate the resolved Compose model
make logs   # follow service logs
make down   # stop and retain state
make clean  # stop and delete state
```
