FROM rust:1.68-alpine3.17 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.17.3

WORKDIR /app
COPY --from=builder /app/target/release/marker /app/main

ENV WEBHOOK_PORT=8080
EXPOSE 8080

ENTRYPOINT ["main"]
