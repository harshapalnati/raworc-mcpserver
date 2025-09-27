# Multi-stage Dockerfile for Raworc MCP Server (HTTP mode for Smithery)

# --- Builder stage ---
FROM rust:1.76-slim AS builder

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    ca-certificates \
    pkg-config \
    libssl-dev \
    build-essential \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache deps
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# --- Runtime stage ---
FROM debian:bookworm-slim AS runtime

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    ca-certificates \
    openssl \
 && rm -rf /var/lib/apt/lists/* \
 && update-ca-certificates

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/raworc-mcp /app/raworc-mcp

# Ensure the binary is executable
RUN chmod +x /app/raworc-mcp

# Default to HTTP mode for Smithery and set sane logging defaults
ENV SERVER_MODE=http \
    RUST_LOG=info

# Smithery will set PORT=8081, but expose for local runs
EXPOSE 8081

CMD ["/app/raworc-mcp"]
