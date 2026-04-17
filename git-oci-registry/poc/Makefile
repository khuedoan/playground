.PHONY: up down init demo verify logs clean push trigger

# Start all services (Zot, Temporal, webhook server, worker)
up:
	docker compose up -d --build
	@echo ""
	@echo "Services starting. Wait ~60s for Temporal to initialize, then run:"
	@echo "  make init"

# Stop all services
down:
	docker compose down

# Initialize k3d cluster with FluxCD (requires k3d, kubectl, flux CLI)
init:
	./scripts/init-k3s.sh

# Run the full demo
demo:
	./scripts/demo.sh

# Push sample manifests to OCI registry (fallback without gnoci)
push:
	@echo "Pushing sample manifests to OCI registry..."
	@TMPDIR=$$(mktemp -d) && \
	cp -r manifests/sample-app/* "$$TMPDIR/" && \
	flux push artifact oci://localhost:5000/demo/myapp:main \
		--path="$$TMPDIR" \
		--source="local" \
		--revision="main@sha1:$$(date +%s | sha256sum | head -c 40)" \
		--insecure-registry && \
	rm -rf "$$TMPDIR" && \
	echo "Done."

# Trigger the CI workflow
trigger:
	curl -s -X POST http://localhost:8080/trigger \
		-H "Content-Type: application/json" \
		-d '{"registry":"zot:5000","repository":"demo/myapp","tag":"main"}' | jq .

# Check deployment status
verify:
	@KUBECONFIG=$$(k3d kubeconfig write poc 2>/dev/null || echo ./kubeconfig.yaml) && \
	export KUBECONFIG && \
	echo "=== OCI Repository ===" && \
	kubectl -n flux-system get ocirepository sample-app -o wide 2>/dev/null || echo "(not found)" && \
	echo "" && \
	echo "=== Kustomization ===" && \
	kubectl -n flux-system get kustomization sample-app -o wide 2>/dev/null || echo "(not found)" && \
	echo "" && \
	echo "=== Pods ===" && \
	kubectl get pods -n default 2>/dev/null || echo "(none)" && \
	echo "" && \
	echo "=== Zot catalog ===" && \
	curl -s http://localhost:5000/v2/_catalog | jq . 2>/dev/null || echo "(registry not reachable)"

# View logs for specific services
logs:
	docker compose logs -f --tail=50

logs-worker:
	docker compose logs -f worker

logs-webhook:
	docker compose logs -f webhook-server

logs-temporal:
	docker compose logs -f temporal

# Clean up everything including volumes and k3d cluster
clean:
	docker compose down -v
	k3d cluster delete poc 2>/dev/null || true
	rm -f kubeconfig.yaml
