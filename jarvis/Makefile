.PHONY: build test run web docker-build docker-run clean

build:
	cargo build --release

test:
	cargo test

run:
	cargo run

# Start the web UI on port 3000
web:
	cargo run -- --web

# Build Docker image (requires network for NixOS base + deps)
docker-build:
	docker build -t jarvis .

# Run in Docker with current directory mounted
docker-run:
	docker run --rm -it \
		-v "$(CURDIR)":/work \
		-e OPENAI_API_KEY \
		jarvis:latest --root /work

# Run in Docker with full isolation (no network, read-only mount)
docker-run-isolated:
	docker run --rm -it \
		--network=none \
		-v "$(CURDIR)":/work:ro \
		-e OPENAI_API_KEY \
		jarvis:latest --root /work --no-approve

clean:
	cargo clean
