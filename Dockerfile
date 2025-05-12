# Stage 1: Build
FROM rust:1.77 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y curl ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copy built binary only
COPY --from=builder /app/target/release/supply_chain_blockchain /app/supply_chain_blockchain

# Let Docker Compose mount config.toml
CMD ["/app/supply_chain_blockchain"]
