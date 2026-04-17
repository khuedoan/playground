# Notes

## 2026-04-17: Initial research

### Search strategy

- GitHub repo search for: "git oci registry backend storage", "oci git remote helper",
  "git-remote-oci", "ORAS git", "git bundle OCI artifact"
- Read READMEs, specs, and open issues for promising projects
- Checked auth/OIDC status specifically

### Key finding: gnoci (act3-ai/gnoci)

The only real implementation. University research project (UDRI/ACT3) that
defines a spec and implements both a git remote helper and LFS transfer agent.

- Go, Apache-2.0, v0.1.1 (Dec 2025)
- Works: clone, fetch, pull, push, ls-remote, tags
- Stores git packfiles as OCI layers, refs as OCI config JSON
- LFS via OCI referrers API
- Auth via Docker credential store (same as docker login)
- Known issues: GHCR push broken (#31), private registry auth buggy (#70),
  no force push (#10), ~20 open issues

### Other projects found

- **git_oras** (zifnab06): Python PoC from 2019, abandoned, not usable
- **git-remote-s3** (mattn): Same pattern but S3 backend, uses git bundles.
  Recent (Apr 2026), 13 stars. Good reference for the remote helper pattern.
- **bento** (kajogo777): Workspace checkpointing to OCI, not git-native

### Auth/OIDC assessment

gnoci delegates to OCI credential chain. No built-in OIDC, but you can wire
OIDC through existing credential helpers (gcloud, az, gh, aws ecr).
For fully keyless (cosign-style) OIDC, a general-purpose OCI credential
helper that does inline OIDC token exchange does not exist yet.

### What does NOT exist

- No spec or project from the OCI community / opencontainers for git-over-OCI
- No integration in mainline Git
- No gitoxide/libgit2 OCI transport
- No server-side "OCI registry that speaks Git protocol"
- The only path is client-side translation via remote helpers
