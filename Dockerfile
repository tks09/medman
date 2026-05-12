# Stage 1: Build the Vue.js frontend
FROM node:18-alpine AS frontend-builder

WORKDIR /frontend

# Copy frontend files
COPY frontend/package*.json ./
RUN npm install

COPY frontend .

# Build the application
RUN npm run build

# Output directory structure: dist -> static
RUN mkdir -p /app/static && cp -r dist/* /app/static/

# Stage 2: Build the Rust backend
FROM rust:1-slim-bookworm AS backend-builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
        pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

RUN cargo build --release

# Stage 3: Final runtime image
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled backend binary
COPY --from=backend-builder /app/target/release/medman-backend ./medman-backend

# Copy the built frontend to static directory
COPY --from=frontend-builder /app/static ./static

EXPOSE 3000

CMD ["./medman-backend"]
