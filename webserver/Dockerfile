FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application with migrations folder bound
COPY . .
RUN --mount=type=bind,source=migrations,target=migrations \
    cargo build --release --bin hello --manifest-path . -- --migrations /app/migrations


# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/hello /usr/local/bin
COPY migrations /app/migrations
ENTRYPOINT ["/usr/local/bin/hello"]