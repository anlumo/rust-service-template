# Leveraging the pre-built Docker images with
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin mygreatservice

# We do not need the Rust toolchain to run the binary!
FROM alpine:latest AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/mygreatservice /usr/local/bin
COPY config.yaml /config.yaml
ENTRYPOINT ["/usr/local/bin/mygreatservice", "--config", "/config.yaml"]
