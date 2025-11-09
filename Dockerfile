# base
FROM rust:1.90-trixie AS base
RUN apt update \
    && apt install -y --no-install-recommends curl ca-certificates gnupg \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

RUN apt update && apt install -y postgresql-client

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs \
    && cargo build --release \
    && rm -rf src target/release/deps/joyus*

COPY package.json package-lock.json* ./
RUN npm ci || npm i --force

# dev
FROM base AS dev
RUN npm install -g concurrently
RUN cargo install cargo-watch
RUN cargo install sqlx-cli --no-default-features --features postgres
COPY . .
EXPOSE 12345
CMD ["./entrypoint.local.sh"]

# prod (builder)
FROM base AS builder
COPY . .
RUN npm run build
RUN cargo build --release

# prod (runtime)
FROM debian:trixie-slim AS runtime
RUN apt update && apt install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/joyus /usr/local/bin/joyus
COPY --from=builder /app/public /app/public
EXPOSE 12345
CMD ["/usr/local/bin/joyus"]
