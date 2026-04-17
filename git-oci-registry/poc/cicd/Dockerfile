FROM golang:1.22-bookworm AS build

WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY *.go ./
RUN CGO_ENABLED=0 go build -o /cicd .

# Install gnoci (git-remote-oci)
RUN go install github.com/act3-ai/gnoci/cmd/git-remote-oci@latest

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    git \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install flux CLI
RUN curl -sL https://fluxcd.io/install.sh | bash

COPY --from=build /cicd /usr/local/bin/cicd
COPY --from=build /go/bin/git-remote-oci /usr/local/bin/

ENTRYPOINT ["cicd"]
