FROM rust:1.72.0-alpine as builder

WORKDIR /app

# Copy only the relevant parts of the mono repo
COPY Cargo.toml Cargo.lock ./

RUN cargo install --path webserver

FROM alpine:3.17

COPY --from=builder /app/.cargo/bin/actix-web /usr/local/bin/actix-web

EXPOSE 8080

# Adjust the command to run your specific Actix Web application
CMD ["actix-web", "run", "--config", "/app/webserver/config.toml"]