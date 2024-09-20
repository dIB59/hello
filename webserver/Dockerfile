# Stage 1: Build the Rust backend
FROM rust:1.71 AS builder

# Install dependencies needed for the build
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Set the working directory to the Rust backend directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock for dependency management
COPY hello/webserver/Cargo.toml hello/webserver/Cargo.lock ./

# Pre-build dependencies to cache them
RUN cargo build --release
RUN rm -rf src/*

# Copy the source code for the backend
COPY hello/webserver/src ./src

# Build the backend for release
RUN cargo build --release

# Stage 2: Prepare a minimal image for running the backend
FROM debian:buster-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled Rust binary from the builder stage
COPY --from=builder /app/target/release/webserver /usr/local/bin/webserver

# Expose the necessary port (e.g., 8080 for Actix)
EXPOSE 8080

# Set the startup command
CMD ["webserver"]
