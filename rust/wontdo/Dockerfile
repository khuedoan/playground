FROM rust:1.58-alpine as builder
WORKDIR /usr/local/src
COPY . .
RUN cargo install --path .

FROM alpine
COPY --from=builder /usr/local/cargo/bin/wontdo /usr/local/bin/wontdo
CMD ["wontdo"]
