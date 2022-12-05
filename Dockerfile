FROM rust:1.65-buster as builder

WORKDIR /app
RUN apt update && apt install pkg-config libssl-dev libc-dev -y
COPY . .
RUN cargo build --release


FROM debian:buster-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates libpq5 \
    # Clean up    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/remote_hut remote_hut
ENTRYPOINT ["./remote_hut"]
