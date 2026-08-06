.DEFAULT_GOAL := up

.PHONY: up down logs check clean

up:
	docker compose up --build

down:
	docker compose down

logs:
	docker compose logs --follow cloudflare-os

check:
	docker compose config --quiet

clean:
	docker compose down --volumes --remove-orphans
