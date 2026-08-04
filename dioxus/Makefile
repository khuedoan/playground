.PHONY: build-css check dev

build-css:
	npm run build:css

check: build-css
	cargo check

dev: build-css
	@npm run watch:css & css_pid=$$!; trap 'kill $$css_pid' EXIT INT TERM; dx serve
