# Git-OCI-Registry Hybrid PoC

Proof of concept for **Option C: gnoci for developers + Flux native OCI for GitOps (hybrid)**.

## Architecture

```
Developer                    Zot Registry              Temporal            k3s + FluxCD
   │                            │                         │                    │
   │ git push (gnoci)           │                         │                    │
   │ ──────────────────────────>│ demo/myapp:main         │                    │
   │                            │                         │                    │
   │ POST /trigger              │                         │                    │
   │ ──────────────────────────>│ webhook-server ────────>│ RepackageWorkflow  │
   │                            │                         │    │               │
   │                            │ gnoci clone <───────────│────┘               │
   │                            │                         │                    │
   │                            │ flux push artifact <────│ myapp-manifests    │
   │                            │                         │                    │
   │                            │ OCIRepository ─────────────────────────────>│ reconcile
   │                            │                         │                    │ deploy
```

## Components (Docker Compose)

| Service | Description |
|---------|-------------|
| **zot** | OCI registry (stores both gnoci git artifacts and Flux OCI artifacts) |
| **temporal** | Temporal server (workflow orchestration) |
| **temporal-db** | PostgreSQL for Temporal |
| **temporal-ui** | Temporal Web UI |
| **webhook-server** | HTTP server that receives triggers and starts Temporal workflows |
| **worker** | Temporal worker that executes the repackage workflow (gnoci clone → flux push) |
| **k3s** | Lightweight Kubernetes with FluxCD |

## Quick Start

```bash
# Start all services
make up

# Wait ~60s for Temporal to initialize, then bootstrap k3s with FluxCD
make init

# Run the full demo
make demo
```

## Manual Walkthrough

```bash
# 1. Start services
docker compose up -d

# 2. Configure gnoci for plain HTTP
mkdir -p ~/.config/gnoci
cat > ~/.config/gnoci/config.yaml << 'EOF'
apiVersion: gnoci.act3-ai.io/v1alpha1
kind: Configuration
registryConfig:
  registries:
    localhost:5000:
      plainHTTP: true
EOF

# 3. Create and push a git repo via gnoci
cd /tmp && git init myapp && cd myapp
mkdir manifests
# ... add k8s manifests ...
git add . && git commit -m "init"
git push --all oci://localhost:5000/demo/myapp:main

# 4. Trigger the CI workflow
curl -X POST http://localhost:8080/trigger \
  -H "Content-Type: application/json" \
  -d '{"registry":"zot:5000","repository":"demo/myapp","tag":"main"}'

# 5. Verify the Flux OCI artifact was created
curl http://localhost:5000/v2/_catalog
# Should show: demo/myapp (gnoci) and demo/myapp-manifests (Flux OCI)
```

## Ports

- `5000` — Zot OCI registry
- `6443` — k3s Kubernetes API
- `7233` — Temporal gRPC
- `8080` — Webhook server
- `8233` — Temporal Web UI

## What Was Verified

1. ✅ gnoci push from host to Zot registry
2. ✅ Webhook trigger → Temporal workflow started
3. ✅ Worker: gnoci clone from OCI → flux push artifact → Flux OCI artifact in Zot
4. ⚠️ k3s FluxCD reconciliation requires image pre-loading in Docker-in-Docker environments

## Notes

- gnoci requires a config file for plain HTTP registries (`~/.config/gnoci/config.yaml`)
- The worker container includes both `git-remote-oci` (gnoci) and `flux` CLI
- Temporal provides durable workflow execution with automatic retries
