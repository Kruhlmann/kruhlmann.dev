FROM rust:1-trixie AS chef

RUN cargo install cargo-chef --locked

FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN cargo build --release

FROM debian:trixie

RUN apt-get update && apt-get install --no-install-recommends --assume-yes ca-certificates libssl3t64 \
    && rm -rf /var/cache/apt/archives /var/lib/apt/lists/* \
    && mkdir -p /usr/share/kruhlmann-dev
COPY --from=builder /app/static/ /usr/share/kruhlmann-dev/static
COPY --from=builder /app/target/release/kruhlmann-dev /usr/local/bin/kruhlmann-dev

ENV STATIC_DIR "/usr/share/kruhlmann-dev/static"

ENTRYPOINT ["/usr/local/bin/kruhlmann-dev"]