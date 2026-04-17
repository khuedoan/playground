.PHONY: up down init demo verify logs clean push trigger

# Start all services
up:
	docker compose up -d --build
	@echo ""
	@echo "Services starting. Wait ~30s for Temporal to initialize, then run:"
	@echo "  make init"

# Stop all services
down:
	docker compose down

# Initialize k3s with FluxCD
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
	@export KUBECONFIG=./kubeconfig.yaml && \
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

# Clean up everything including volumes
clean:
	docker compose down -v
	rm -f kubeconfig.yaml
