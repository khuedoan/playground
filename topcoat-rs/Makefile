.PHONY: build css dev test

build: css
	cargo build

css:
	npm run build:css

dev:
	npm run dev:css & trap 'kill $$!' EXIT INT TERM; topcoat dev

test: css
	cargo test
