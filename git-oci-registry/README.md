# Git over OCI Container Registry

## Question

Can you use an OCI container registry as the backend for standard Git
operations (push, pull, tag, clone, etc.) with standard authentication,
ideally passwordless OIDC?

## TL;DR

**Yes — [`gnoci`](https://github.com/act3-ai/gnoci) does exactly this.** It
is a Git remote helper that lets you use `git push`, `git pull`, `git clone`,
`git tag`, etc. with any OCI-compliant registry as the remote. It is early
(v0.1.1, Dec 2025) but functional, written in Go, and Apache-2.0 licensed.

Authentication currently uses the Docker/OCI credential store (same as
`docker login`), so any registry that supports Docker-style auth works. OIDC
token exchange is not built into `gnoci` itself but can be wired in through
the existing OCI credential helpers or registry-side OIDC (e.g. GHCR with
`gh auth`, ACR with `az acr login`, etc.).

## Existing Projects

### 1. gnoci (act3-ai/gnoci) — ⭐ Best fit

| | |
|---|---|
| **Repo** | <https://github.com/act3-ai/gnoci> |
| **Language** | Go |
| **License** | Apache-2.0 |
| **Latest release** | v0.1.1 (2025-12-10) |
| **Stars** | 3 |
| **Status** | Active development (university research project at UDRI) |

#### What it does

- Implements a **Git remote helper** (`git-remote-oci`) so Git natively
  understands `oci://` URLs.
- Also implements a **Git LFS custom transfer agent**
  (`git-lfs-remote-oci`) for large files.
- Defines a formal **OCI artifact spec** for storing Git repos: packfiles
  as OCI layers, refs as OCI config JSON.
- Supports all standard operations: clone, fetch, pull, push, ls-remote,
  tagging.

#### How to use

```bash
# Install
go install github.com/act3-ai/gnoci/cmd/git-remote-oci@latest

# Push an existing repo
git push --all oci://ghcr.io/myorg/myrepo:main

# Clone from OCI
git clone oci://ghcr.io/myorg/myrepo:main

# Add as a named remote
git remote add oci-origin oci://ghcr.io/myorg/myrepo:main
git push oci-origin main
git pull oci-origin main

# Tags
git tag v1.0.0
git push oci-origin --tags
```

#### Data model

- Git packfiles stored as OCI image layers
  (`application/vnd.ai.act3.git.pack.v1`)
- Git refs (branches, tags) stored in OCI config JSON
  (`application/vnd.ai.act3.git.config.v1+json`)
- LFS objects stored as referrer artifacts via the OCI Referrers API
- Follows OCI image-spec artifact guidelines (decision tree #3)
- Incremental: subsequent pushes create thin packfile layers, not full
  re-uploads

#### Current limitations (open issues)

- **Private registry auth** is buggy (issue #70) — they have the credential
  helper wired but it needs fixes.
- **GHCR push** has issues with HTTP redirect handling (issue #31).
- **No force push** yet (issue #10).
- **Data races** on concurrent pushes to same tag (issue #36).
- **Manifest size limits** — no packfile merging yet when many layers
  accumulate (issue #35).
- ~20 open issues, actively being worked on.

### 2. git_oras (zifnab06/git_oras) — Proof of concept, abandoned

| | |
|---|---|
| **Repo** | <https://github.com/zifnab06/git_oras> |
| **Language** | Python |
| **Created** | 2019-08-10 |
| **Stars** | 0 |
| **Status** | Abandoned (no updates since 2019) |

Early proof-of-concept using ORAS (OCI Registry as Storage) library as a
Git plugin. Predates the modern OCI artifact spec. Not usable today.

### 3. git-remote-s3 (mattn/git-remote-s3) — Similar pattern, S3 backend

| | |
|---|---|
| **Repo** | <https://github.com/mattn/git-remote-s3> |
| **Language** | Go |
| **Created** | 2026-04-05 |
| **Stars** | 13 |
| **Status** | Active |

Uses the same Git remote helper approach but with S3 (including
S3-compatible stores like MinIO and OCI Object Storage) as the backend.
Stores `git bundle` snapshots rather than packfiles. Relevant as a
reference implementation for the remote helper pattern, and compatible
with S3-API registries, but not OCI registries.

### 4. bento (kajogo777/bento) — Adjacent, not Git-native

| | |
|---|---|
| **Repo** | <https://github.com/kajogo777/bento> |
| **Language** | Go |
| **Created** | 2026-03-27 |
| **Stars** | 11 |
| **Status** | Active |

Checkpoints entire workspaces (code + deps + agent memory) as layered OCI
artifacts pushed to any container registry. Not a Git remote — it's a
parallel tool. Interesting for the "push workspace to OCI" concept but
does not integrate with `git push/pull`.

## Authentication & OIDC

### How gnoci handles auth today

`gnoci` uses the standard OCI/Docker credential chain via the `oras`
Go library:

1. **Docker credential store** (`~/.docker/config.json`) — same creds as
   `docker login`.
2. Any **Docker credential helpers** configured (e.g.
   `docker-credential-gcloud`, `docker-credential-ecr-login`).

### Path to passwordless OIDC

Since `gnoci` delegates auth to the OCI credential chain, you can achieve
passwordless OIDC by configuring the credential helper for your registry:

| Registry | Passwordless OIDC method |
|---|---|
| **GHCR** | `gh auth login` (device flow or OIDC), then `gh auth token` feeds creds to Docker config |
| **ACR (Azure)** | `az acr login` uses Azure AD / Entra ID OIDC tokens |
| **GCR / Artifact Registry** | `gcloud auth configure-docker` uses Google OIDC |
| **ECR (AWS)** | `aws ecr get-login-password` with IRSA/OIDC federation |
| **Harbor** | Supports OIDC providers directly |
| **Zot** | Supports OpenID Connect authentication |
| **Generic** | Use `crane auth login` with a token obtained from any OIDC flow |

In CI (GitHub Actions), the standard `docker/login-action` with OIDC
token would work transparently since `gnoci` reads the same credential
store.

### What's missing for first-class OIDC

- `gnoci` does not have its own `--oidc-issuer` flag or built-in token
  exchange — it fully delegates to the OCI auth layer.
- This is arguably the right design: separation of concerns.
- For a fully keyless workflow (like cosign's keyless signing), someone
  would need to implement an OCI credential helper that does OIDC
  discovery + token exchange inline. This does not exist as a
  general-purpose tool yet.

## Architecture: How Git Remote Helpers Work

The mechanism that makes this possible is Git's
[remote helper](https://git-scm.com/docs/gitremote-helpers) protocol:

1. You install a binary named `git-remote-<scheme>` on your `$PATH`.
2. When Git encounters a URL like `<scheme>://...`, it spawns the helper.
3. The helper communicates with Git over stdin/stdout using a text
   protocol (capabilities, list, fetch, push commands).
4. The helper translates between Git's object model and the storage
   backend.

This is the same mechanism used by `git-remote-http`,
`git-remote-ftp`, etc. `gnoci` implements `git-remote-oci`.

## Recommendation

**Use `gnoci` (`act3-ai/gnoci`).** It is the only project that provides
a real Git remote helper for OCI registries with a formal spec.

Steps to try it:

1. `go install github.com/act3-ai/gnoci/cmd/git-remote-oci@latest`
2. Ensure `git-remote-oci` is on `$PATH`
3. `docker login` to your registry (or use credential helpers for OIDC)
4. `git clone oci://your-registry.io/org/repo:main`
5. Make changes, commit, `git push`

Be aware it is early-stage software with known issues around GHCR
compatibility and private registry auth. For production use, test
thoroughly. For air-gapped or controlled environments (its original
use case with Zarf), it should work well with registries like Zot or
Harbor.

## References

- gnoci repo: <https://github.com/act3-ai/gnoci>
- gnoci OCI spec: <https://github.com/act3-ai/gnoci/blob/main/docs/spec/oci-spec.md>
- Git remote helpers docs: <https://git-scm.com/docs/gitremote-helpers>
- ORAS project: <https://oras.land/>
- OCI Distribution Spec: <https://github.com/opencontainers/distribution-spec>
- OCI Image Spec (artifacts): <https://github.com/opencontainers/image-spec>
