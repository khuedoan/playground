# Headscale playground

https://headscale.net/running-headscale-container/#configure-and-run-headscale

```sh
docker compose up
docker compose exec headscale headscale users create testuser
docker compose exec headscale headscale --user testuser preauthkeys create --reusable --expiration 24h
```
