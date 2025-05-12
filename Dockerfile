# Stage 1: Build using a consistent base to avoid GLIBC issues
FROM rust:1.77-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y build-essential pkg-config libssl-dev

WORKDIR /app
COPY . .

# Build in release mode
RUN cargo build --release

# Stage 2: Runtime with same base to ensure compatibility
FROM rust:1.77-slim

RUN apt-get update && apt-get install -y curl ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copy compiled binary from builder
COPY --from=builder /app/target/release/supply_chain_blockchain /app/supply_chain_blockchain

# Let Docker Compose mount config.toml at runtime
CMD ["/app/supply_chain_blockchain"]
