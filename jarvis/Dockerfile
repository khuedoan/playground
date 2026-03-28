# jarvis — NixOS-based Docker image
#
# Multi-stage build:
#   1. Build stage: compile the Rust binary
#   2. Runtime stage: NixOS-based image with the binary + minimal sandbox tools
#
# Build:  docker build -t jarvis .
# Run:    docker run --rm -it -e OPENAI_API_KEY -v "$(pwd)":/work jarvis --root /work
#
# For full isolation (untrusted repos):
#   docker run --rm -it --network=none -v "$(pwd)":/work:ro -e OPENAI_API_KEY jarvis --root /work
#
# Alternative pure-Nix build (requires Nix on host):
#   nix build .#dockerImages.x86_64-linux.default && docker load < result

# ── Stage 1: Build ──────────────────────────────────────────────────────
FROM nixos/nix:latest AS builder

RUN mkdir -p /etc/nix \
    && echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

WORKDIR /src
COPY . .

# Install build tools via Nix and compile
RUN nix profile install \
      nixpkgs#cargo \
      nixpkgs#rustc \
      nixpkgs#pkg-config \
      nixpkgs#openssl \
    && cargo build --release \
    && cp target/release/jarvis /jarvis

# ── Stage 2: NixOS runtime ──────────────────────────────────────────────
FROM nixos/nix:latest

RUN mkdir -p /etc/nix \
    && echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

# Install only the minimal tools the agent sandbox needs.
# These are the commands on the default allowlist.
RUN nix profile install \
      nixpkgs#coreutils \
      nixpkgs#gnugrep \
      nixpkgs#findutils \
      nixpkgs#diffutils \
      nixpkgs#git \
      nixpkgs#cacert \
    && nix-collect-garbage -d

COPY --from=builder /jarvis /usr/local/bin/jarvis

RUN mkdir -p /work
WORKDIR /work

ENV SSL_CERT_FILE=/nix/var/nix/profiles/default/etc/ssl/certs/ca-bundle.crt

ENTRYPOINT ["jarvis"]
CMD ["--help"]
