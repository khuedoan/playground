# Git-OCI-Registry Hybrid PoC — Notes

## 2026-04-17: Initial PoC

### Objective
Demonstrate Option C: gnoci for developers + Flux native OCI for GitOps (hybrid),
using Temporal workflows for CI/CD orchestration.

### Architecture decisions
- **Zot** as the OCI registry — lightweight, supports OCI artifacts natively
- **Temporal** for CI/CD — durable workflows with retries, visibility via Web UI
- **gnoci (git-remote-oci)** for developer git push to OCI
- **flux push artifact** for creating Flux-native OCI artifacts
- **k3s** as lightweight Kubernetes with FluxCD

### What works
1. gnoci push from host to Zot (`git push --all oci://localhost:5000/demo/myapp:main`)
2. Webhook trigger → Temporal RepackageWorkflow starts
3. Worker: gnoci clone from OCI → extract manifests → flux push artifact to Zot
4. Both `demo/myapp` (gnoci) and `demo/myapp-manifests` (Flux OCI) visible in Zot catalog
5. FluxCD OCIRepository + Kustomization manifests configured

### Key findings

#### gnoci configuration
- gnoci requires a YAML config file for plain HTTP registries
- Location: `~/.config/gnoci/config.yaml`
- Format:
  ```yaml
  apiVersion: gnoci.act3-ai.io/v1alpha1
  kind: Configuration
  registryConfig:
    registries:
      localhost:5000:
        plainHTTP: true
  ```

#### Flux CLI versions
- `flux push artifact --insecure-registry` requires flux v2.5+ (v2.4.0 does NOT have this flag)
- Using v2.8.5 in this PoC

#### k3s / Kubernetes in Docker
- Running k3s directly in Docker Compose does NOT work for pod networking:
  - Kubelet readiness probes can't reach pod IPs (SYN packets unreplied)
  - ClusterIP DNAT doesn't work correctly (CoreDNS can't reach API server)
  - Root cause: iptables conntrack/NAT in nested Docker lacks proper kernel module support
  - Tried: `--flannel-backend=vxlan` (no vxlan module), `--flannel-backend=host-gw`, `--disable-network-policy`, bridge iptables fixes — none worked
  - Also tried Kind (same environment, same failure)
- **Solution**: Use k3d (k3s-in-Docker purpose-built tool) to create the cluster externally,
  then connect it to the Docker Compose network via `--network`
- k3d on a real Docker host (not nested) works perfectly
- Removed k3s from docker-compose.yml; init script now uses k3d

#### Temporal
- `temporalio/auto-setup:1.25.2` takes ~30-60s to initialize the DB schema on first start
- Worker and webhook-server retry connecting to Temporal (30 attempts, 2s interval)

### Failed approaches
- k3s in Docker Compose — pod networking broken in nested Docker (see detailed notes above)
- Kind in nested Docker — same iptables/conntrack issue
- `--flannel-backend host-gw` — doesn't fix the conntrack issue
- `--flannel-backend vxlan` — requires vxlan kernel module not available in some CI envs
- `flux push artifact` without `--insecure-registry` — fails for plain HTTP registries
- Older flux CLI (v2.4.0) — does not support `--insecure-registry` flag

### TODO
- Zot webhook notifications for automatic CI trigger on push (currently manual POST to /trigger)
- Container image building as part of the CI workflow
- Multi-environment promotion (dev → staging → prod tags)
- Flux notification controller for deployment status callbacks
