FROM rust:alpine3.18 as builder
WORKDIR /
COPY src src
COPY cargo.toml Cargo.toml
RUN apk --no-cache add musl-dev && \
    cargo build --release
FROM alpine:3.18
COPY --from=builder /target/release/image2ascii /usr/local/bin/image2ascii