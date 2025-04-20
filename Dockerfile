# -------- Stage 1: Build Static Binary --------
FROM rustlang/rust:nightly-slim as builder

# Install MUSL cross toolchain
RUN apt-get update && apt-get install -y musl-tools

# Add MUSL target to Rust
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Copy everything
COPY . .

# Build static binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# -------- Stage 2: Runtime --------
FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/agent_token_gateway .
COPY .env .env

EXPOSE 3000

CMD ["./agent_token_gateway"]
