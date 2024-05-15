# Set base image
FROM rust:1.78-bookworm as builder

WORKDIR /usr/src/app

COPY . .

# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/youkoso_auth ./youkoso_auth

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y libpq-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

#Run
COPY --from=builder /usr/src/app/youkoso_auth /app/youkoso_auth

# Run apps
CMD ./youkoso_auth
