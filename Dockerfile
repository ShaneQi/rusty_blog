# Build stage — rust:1 tracks current stable
FROM rust:1-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends cmake pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release \
    && strip target/release/rusty_blog

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/rusty_blog /usr/local/bin/rusty_blog
COPY templates ./templates
COPY assets ./assets

# Args: <content_input_path> <output_path>
ENTRYPOINT ["rusty_blog"]
CMD ["/content", "/output"]
