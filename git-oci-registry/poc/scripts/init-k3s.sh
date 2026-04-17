#!/usr/bin/env bash
# Initialize a k3d cluster with FluxCD, connected to the Docker Compose network.
# Run this after 'docker compose up -d' once Temporal is healthy.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
POC_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# ── Prerequisites ────────────────────────────────────────────────────
for cmd in k3d kubectl flux; do
  if ! command -v "$cmd" &>/dev/null; then
    echo "ERROR: $cmd is required but not found in PATH"
    exit 1
  fi
done

# ── Determine the Docker Compose network ─────────────────────────────
COMPOSE_PROJECT=$(cd "$POC_DIR" && docker compose config --format json 2>/dev/null | grep -o '"name":"[^"]*"' | head -1 | cut -d'"' -f4)
COMPOSE_PROJECT="${COMPOSE_PROJECT:-poc}"
NETWORK="${COMPOSE_PROJECT}_default"

echo "==> Using Docker network: $NETWORK"

# Get Zot registry IP on the compose network (for k3s registries config)
ZOT_IP=$(docker inspect "$(cd "$POC_DIR" && docker compose ps -q zot)" --format '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' 2>/dev/null || echo "")
echo "    Zot registry IP: ${ZOT_IP:-unknown}"

# ── Create k3d cluster ───────────────────────────────────────────────
CLUSTER_NAME="poc"

if k3d cluster list 2>/dev/null | grep -q "$CLUSTER_NAME"; then
  echo "==> k3d cluster '$CLUSTER_NAME' already exists"
else
  echo "==> Creating k3d cluster '$CLUSTER_NAME' on network '$NETWORK'..."
  k3d cluster create "$CLUSTER_NAME" \
    --no-lb \
    --k3s-arg "--disable=traefik@server:0" \
    --k3s-arg "--disable=metrics-server@server:0" \
    --k3s-arg "--disable-network-policy@server:0" \
    --network "$NETWORK" \
    --wait \
    --timeout 180s
fi

export KUBECONFIG=$(k3d kubeconfig write "$CLUSTER_NAME")
echo "    KUBECONFIG=$KUBECONFIG"

# ── Configure k3s to use Zot as an insecure registry ─────────────────
K3D_NODE="k3d-${CLUSTER_NAME}-server-0"
if [ -n "$ZOT_IP" ]; then
  echo "==> Configuring k3s to use Zot registry at $ZOT_IP:5000..."
  docker exec "$K3D_NODE" sh -c "mkdir -p /etc/rancher/k3s && cat > /etc/rancher/k3s/registries.yaml" <<EOF
mirrors:
  "zot:5000":
    endpoint:
      - "http://${ZOT_IP}:5000"
configs:
  "zot:5000":
    tls:
      insecure_skip_verify: true
EOF
fi

# ── Wait for cluster readiness ────────────────────────────────────────
echo "==> Waiting for k3d cluster to be ready..."
for i in $(seq 1 60); do
  if docker exec "$K3D_NODE" kubectl get nodes &>/dev/null 2>&1; then
    break
  fi
  echo "    Attempt $i/60..."
  sleep 3
done
docker exec "$K3D_NODE" kubectl wait --for=condition=Ready node --all --timeout=180s
echo "    Cluster ready"

# ── Wait for system pods ──────────────────────────────────────────────
echo "==> Waiting for system pods..."
for i in $(seq 1 60); do
  READY=$(docker exec "$K3D_NODE" kubectl -n kube-system get pods -o jsonpath='{.items[?(@.metadata.labels.k8s-app=="kube-dns")].status.conditions[?(@.type=="Ready")].status}' 2>/dev/null || echo "False")
  if [ "$READY" = "True" ]; then
    echo "    CoreDNS is ready"
    break
  fi
  if [ "$i" -eq 60 ]; then
    echo "    WARNING: CoreDNS not ready after 3 minutes, continuing anyway..."
  fi
  sleep 3
done

# ── Install FluxCD ────────────────────────────────────────────────────
echo "==> Installing FluxCD..."
flux install \
  --components=source-controller,kustomize-controller \
  --timeout=5m \
  --kubeconfig="$KUBECONFIG"
echo "    FluxCD installed"

echo "==> Waiting for FluxCD controllers to be ready..."
kubectl --kubeconfig="$KUBECONFIG" -n flux-system wait --for=condition=Ready pod --all --timeout=300s
echo "    FluxCD controllers ready"

# ── Apply Flux sources ────────────────────────────────────────────────
echo "==> Applying OCIRepository and Kustomization..."
kubectl --kubeconfig="$KUBECONFIG" apply -f "$POC_DIR/manifests/flux-source.yaml"
kubectl --kubeconfig="$KUBECONFIG" apply -f "$POC_DIR/manifests/flux-kustomization.yaml"
echo "    Flux resources applied"

echo ""
echo "==> Initialization complete!"
echo "    KUBECONFIG=$KUBECONFIG"
echo "    Temporal UI: http://localhost:8233"
echo "    Webhook:     http://localhost:8080/trigger (POST)"
echo "    OCI Registry: localhost:5000"
echo ""
echo "    To use kubectl: export KUBECONFIG=$KUBECONFIG"
