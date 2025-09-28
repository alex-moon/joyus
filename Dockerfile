# Multi-stage Dockerfile for Joyus (Rust + Webpack)
# Provides dev and prod targets.

# 1) Base image with Rust and Node (for building web assets)
FROM rust:1.90-trixie AS base

# Install Node.js (via Debian repo) and build tooling
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl ca-certificates gnupg \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Pre-cache Rust deps (improve incremental builds)
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs \
    && cargo build --release \
    && rm -rf src target/release/deps/joyus*

# Pre-cache Node deps
COPY package.json package-lock.json* ./
RUN npm ci || npm i --force

# 2) Development stage: installs cargo-watch and runs concurrent dev servers
FROM base AS dev
# Install concurrently and cargo-watch for dev hot-reload
RUN npm install -g concurrently
RUN cargo install cargo-watch
# Copy the rest of the sources
COPY . .
EXPOSE 12345
# Default dev command (webpack in watch + cargo watch -x run)
CMD ["npm", "run", "dev"]

# 3) Production builder: build static assets and Rust binary
FROM base AS builder
COPY . .
# Build frontend (webpack production)
RUN npm run build
# Build Rust binary
RUN cargo build --release

# 4) Production runtime: minimal image with the compiled binary and built assets
FROM debian:trixie-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
# Copy Rust binary
COPY --from=builder /app/target/release/joyus /usr/local/bin/joyus
# Copy built web assets and generated index.html
COPY --from=builder /app/public /app/public
# Expose app port
EXPOSE 12345
# Run the app
CMD ["/usr/local/bin/joyus"]
