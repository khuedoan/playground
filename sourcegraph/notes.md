# Notes

## 2026-03-28

- Imported the upstream Sourcegraph `7.0.2852` Docker Compose stack into this
  experiment folder with a smaller `compose.override.yaml`.
- Fixed the local Postgres memory limits so `pgsql` and `codeintel-db` have
  headroom above their configured `shared_buffers = 1GB`.
- Removed `trusted_proxies 0.0.0.0/0` from all built-in Caddy templates so the
  default local stack does not trust spoofed `X-Forwarded-*` headers.
- Removed the broken Jaeger datasource from Grafana provisioning and added
  `deleteDatasources` so an already-provisioned stale datasource is deleted on
  restart.
- Found an additional Colima-specific runtime issue: `zoekt-webserver-0`
  failed because the image starts it through `/sbin/tini -s`. The local
  override now invokes `zoekt-webserver` directly with the same flags.
- Verified:
  - `docker compose config`
  - `docker compose up -d`
  - `docker compose restart caddy grafana`
  - `curl -I http://127.0.0.1/`
  - `docker compose exec zoekt-webserver-0 wget -q 'http://127.0.0.1:6070/healthz' -O -`
- Tried disabling cadvisor CPU metrics with
  `-disable_metrics=cpu,cpuLoad,percpu`, but it still failed with the same CPU
  clock-speed detection error on ARM Colima.
