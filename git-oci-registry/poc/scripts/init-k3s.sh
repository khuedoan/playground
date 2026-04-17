#!/usr/bin/env bash
# Initialize k3s with FluxCD and configure the OCIRepository source.
# Run this after 'docker compose up -d' once k3s is healthy.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
POC_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Extract kubeconfig from the k3s volume
echo "==> Extracting kubeconfig from k3s..."
KUBECONFIG_FILE="$POC_DIR/kubeconfig.yaml"
docker compose -f "$POC_DIR/docker-compose.yml" cp k3s:/output/kubeconfig.yaml "$KUBECONFIG_FILE"
# Rewrite the server address to localhost (accessible from the host)
sed -i 's|https://127.0.0.1:6443|https://localhost:6443|g' "$KUBECONFIG_FILE"
export KUBECONFIG="$KUBECONFIG_FILE"

echo "==> Waiting for k3s to be ready..."
until kubectl get nodes &>/dev/null; do
  sleep 2
done
kubectl wait --for=condition=Ready node --all --timeout=120s
echo "    k3s is ready"

echo "==> Installing FluxCD..."
if ! command -v flux &>/dev/null; then
  echo "    flux CLI not found, installing..."
  curl -sL https://fluxcd.io/install.sh | bash
fi
flux install --components=source-controller,kustomize-controller --timeout=5m
echo "    FluxCD installed"

echo "==> Applying OCIRepository and Kustomization..."
kubectl apply -f "$POC_DIR/manifests/flux-source.yaml"
kubectl apply -f "$POC_DIR/manifests/flux-kustomization.yaml"
echo "    Flux resources applied"

echo ""
echo "==> Initialization complete!"
echo "    KUBECONFIG=$KUBECONFIG_FILE"
echo "    Temporal UI: http://localhost:8233"
echo "    Webhook:     http://localhost:8080/trigger (POST)"
echo "    OCI Registry: localhost:5000"
