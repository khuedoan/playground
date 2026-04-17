FROM golang:1.24-bookworm AS build

WORKDIR /app
COPY cicd/go.mod cicd/go.sum ./
RUN go mod download
COPY cicd/*.go ./
RUN CGO_ENABLED=0 go build -o /cicd .

# Install gnoci (git-remote-oci) — requires Go 1.25+, auto-download via GOTOOLCHAIN
RUN GOTOOLCHAIN=auto go install github.com/act3-ai/gnoci/cmd/git-remote-oci@latest

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    git \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install flux CLI (direct download from GitHub)
RUN curl -sLo /tmp/flux.tar.gz https://github.com/fluxcd/flux2/releases/download/v2.8.5/flux_2.8.5_linux_amd64.tar.gz && \
    tar xzf /tmp/flux.tar.gz -C /usr/local/bin flux && \
    rm /tmp/flux.tar.gz && \
    flux --version

COPY --from=build /cicd /usr/local/bin/cicd
COPY --from=build /go/bin/git-remote-oci /usr/local/bin/

# gnoci config for plain HTTP registries
RUN mkdir -p /root/.config/gnoci
COPY configs/gnoci-config.yaml /root/.config/gnoci/config.yaml

ENTRYPOINT ["cicd"]
