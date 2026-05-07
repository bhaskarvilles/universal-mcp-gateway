# Multi-stage build for Universal MCP Gateway

# Stage 1: Build
FROM rust:1.75-slim as builder

WORKDIR /build

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY gateway-core ./gateway-core

# Build release binary
RUN cargo build --release --bin mcp-gateway

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/target/release/mcp-gateway /app/mcp-gateway

# Copy default config
COPY examples/config.yaml /app/config.yaml.example

# Create non-root user
RUN useradd -m -u 1000 gateway && \
    chown -R gateway:gateway /app

USER gateway

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

ENTRYPOINT ["/app/mcp-gateway"]
CMD ["--config", "/app/config.yaml"]
