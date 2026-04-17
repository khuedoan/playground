#!/usr/bin/env bash
# End-to-end demo of the hybrid git-oci-registry PoC.
# Prerequisites: docker compose up -d && ./scripts/init-k3s.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
POC_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

export KUBECONFIG="$POC_DIR/kubeconfig.yaml"

echo "============================================"
echo " Git-OCI-Registry Hybrid PoC Demo"
echo "============================================"
echo ""

# ── Step 1: Create a sample git repo with k8s manifests ──────────────
echo "==> Step 1: Create sample git repo with k8s manifests"
WORK_DIR=$(mktemp -d)
trap "rm -rf $WORK_DIR" EXIT

cd "$WORK_DIR"
git init sample-app
cd sample-app
mkdir -p manifests
cp "$POC_DIR/manifests/sample-app/deployment.yaml" manifests/
git add .
git commit -m "Initial commit: nginx deployment"

echo "    Created sample repo at $WORK_DIR/sample-app"
echo ""

# ── Step 2: Push to OCI registry via gnoci ────────────────────────────
echo "==> Step 2: Push to OCI registry via gnoci"
if command -v git-remote-oci &>/dev/null; then
  git push --all oci://localhost:5000/demo/myapp:main
  echo "    Pushed via gnoci to oci://localhost:5000/demo/myapp:main"
else
  echo "    gnoci not installed on host — using flux push artifact directly as fallback"
  flux push artifact oci://localhost:5000/demo/myapp:main \
    --path="./manifests" \
    --source="local" \
    --revision="main@sha1:$(git rev-parse HEAD)" \
    --insecure-registry
  echo "    Pushed manifests to oci://localhost:5000/demo/myapp:main"
fi
echo ""

# ── Step 3: Trigger CI workflow via webhook ───────────────────────────
echo "==> Step 3: Trigger CI workflow via webhook server"
RESPONSE=$(curl -s -X POST http://localhost:8080/trigger \
  -H "Content-Type: application/json" \
  -d '{"registry":"zot:5000","repository":"demo/myapp","tag":"main"}')
echo "    Webhook response: $RESPONSE"

WORKFLOW_ID=$(echo "$RESPONSE" | grep -o '"workflowID":"[^"]*"' | cut -d'"' -f4)
echo "    Workflow ID: $WORKFLOW_ID"
echo ""

# ── Step 4: Wait for workflow completion ──────────────────────────────
echo "==> Step 4: Waiting for Temporal workflow to complete..."
for i in $(seq 1 60); do
  sleep 2
  # Check if the flux artifact exists in zot
  if curl -sf http://localhost:5000/v2/demo/myapp-manifests/tags/list &>/dev/null; then
    echo "    Flux OCI artifact published!"
    break
  fi
  if [ "$i" -eq 60 ]; then
    echo "    Timeout waiting for workflow. Check Temporal UI at http://localhost:8233"
    exit 1
  fi
done
echo ""

# ── Step 5: Wait for FluxCD to reconcile ─────────────────────────────
echo "==> Step 5: Waiting for FluxCD to reconcile..."
sleep 5  # give flux time to pick up the new artifact
flux reconcile source oci sample-app --timeout=2m 2>/dev/null || true
flux reconcile kustomization sample-app --timeout=2m 2>/dev/null || true

echo "    Waiting for deployment..."
kubectl wait --for=condition=Available deployment/sample-app \
  --namespace=default --timeout=120s 2>/dev/null && \
  echo "    ✅ sample-app deployed successfully!" || \
  echo "    ⚠ Deployment not ready yet — check 'kubectl get pods'"
echo ""

# ── Step 6: Verify ───────────────────────────────────────────────────
echo "==> Step 6: Verification"
echo ""
echo "Flux OCIRepository status:"
kubectl -n flux-system get ocirepository sample-app -o wide 2>/dev/null || echo "  (not ready yet)"
echo ""
echo "Flux Kustomization status:"
kubectl -n flux-system get kustomization sample-app -o wide 2>/dev/null || echo "  (not ready yet)"
echo ""
echo "Pods:"
kubectl get pods -n default 2>/dev/null || echo "  (none yet)"
echo ""
echo "============================================"
echo " Demo complete!"
echo "============================================"
