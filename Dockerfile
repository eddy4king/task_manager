# Stage 1 — Build
FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx

ENV SQLX_OFFLINE=true

RUN cargo build --release

# Stage 2 — Runtime
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/task_manager .
COPY migrations ./migrations

EXPOSE 3000

CMD ["./task_manager"]