.POSIX:
.PHONY: check fmt setup test

check:
	cargo clippy --workspace --all-targets -- -D warnings
	cargo test --workspace
	cd apps/control_plane && mix compile --warnings-as-errors
	cd apps/control_plane && mix test
	nix flake check

fmt:
	cargo fmt --all
	cd apps/control_plane && mix format
	nix fmt

setup:
	cd apps/control_plane && mix setup

test:
	cargo test --workspace
	cd apps/control_plane && mix test
