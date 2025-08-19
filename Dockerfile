FROM rust:1.82 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY static ./static

RUN cargo build --release --bin server

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/server ./server
COPY --from=builder /app/static ./static

EXPOSE 3000

CMD ["./server"]