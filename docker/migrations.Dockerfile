FROM rust:1.85-slim-bookworm

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install sqlx-cli --no-default-features --features postgres

COPY crates/borer-server/migrations ./migrations

CMD ["sqlx", "migrate", "run"]

