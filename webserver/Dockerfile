# Use the official Rust image to build the project
FROM rust:1.72 as builder

# Set the working directory inside the container
WORKDIR /usr/src/webserver

# Copy the Cargo.toml and Cargo.lock files to install dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to get dependencies cached
RUN echo "fn main() {}" > src/main.rs

# Fetch dependencies (cached)
RUN cargo build --release

# Copy the rest of the code
COPY . .

# Build the actual application
RUN cargo build --release

# Now use a small image for production (e.g., alpine-based image)
FROM debian:buster-slim

# Install necessary OS dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /usr/src/webserver

# Copy the compiled binary from the build image
COPY --from=builder /usr/src/webserver/target/release/webserver .

# Expose the port your app runs on
EXPOSE 8080

# Set environment variables if needed
# ENV RUST_LOG=info

# Run the web server
CMD ["./webserver"]
