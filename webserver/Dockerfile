# Use the official Rust image as the base for building
FROM rust:1.72 AS builder

# Set the working directory inside the Docker container
WORKDIR /app

# Copy the manifest files to cache dependencies
COPY webserver/Cargo.toml webserver/Cargo.lock ./
COPY webserver/diesel.toml ./

# Create a dummy source file to trigger the caching of dependencies
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

# Fetch dependencies
RUN cargo fetch

# Copy the entire source code
COPY webserver/ .

# Compile the project in release mode
RUN cargo build --release

# Use a smaller base image for the final stage
FROM debian:buster-slim

# Install system dependencies, including Diesel CLI if necessary for migrations
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the previous stage
COPY --from=builder /app/target/release/webserver .

# Copy the Diesel migrations folder
COPY webserver/migrations ./migrations

# Expose the port that your Actix web app runs on
EXPOSE 8080

# Command to run your Actix web server
CMD ["./webserver"]
