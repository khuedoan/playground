.PHONY: run test fmt e2e

run:
	cargo run -- --listen 127.0.0.1:8080 --repos-root ./repos

test:
	cargo test

fmt:
	cargo fmt

e2e:
	./scripts/e2e.sh
