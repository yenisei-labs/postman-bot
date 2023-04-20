FROM rust:1.68-slim-bullseye AS builder

RUN apt update && apt install -y pkg-config libssl-dev

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /app/target/release/postman-bot /app/main

ENV WEBHOOK_PORT=8080
EXPOSE 8080

ENTRYPOINT ["/app/main"]
