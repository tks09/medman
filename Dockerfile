# Rust backend Dockerfile
FROM rust:1.75-slim as builder

WORKDIR /app

# Copy cargo config to speed up builds
COPY Cargo.toml ./
COPY Cargo.lock ./

# Build in release mode
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder
COPY --from=builder /app/target/release/medman-backend ./

# Copy environment file
COPY .env ./

EXPOSE 3000

CMD ["./medman-backend"]
