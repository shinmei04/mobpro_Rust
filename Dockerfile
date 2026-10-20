FROM rust:1-bookworm

RUN rustup component add rustfmt clippy

ENV CARGO_HOME=/cargo \
    CARGO_TARGET_DIR=/target \
    PATH=/cargo/bin:${PATH}

WORKDIR /workspace

# Keep the development container running; execute Cargo with docker compose exec.
CMD ["sleep", "infinity"]
