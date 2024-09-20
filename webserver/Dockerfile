FROM rust:1.81.0-alpine as builder

WORKDIR /app

COPY webserver/Cargo.toml webserver/Cargo.lock ./

RUN cargo install cargo-edit
RUN cargo add actix-web

COPY webserver/src/ ./src/

RUN cargo build --release

FROM alpine:3.16

WORKDIR /app

# Adjust the binary name to match the output of your project
COPY --from=builder /app/target/release/hello.exe /app/hello.exe
EXPOSE 8080
CMD ["/app/hello.exe"]
