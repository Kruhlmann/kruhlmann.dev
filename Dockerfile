FROM rust:1-trixie AS chef

RUN cargo install cargo-chef --locked

FROM chef AS planner
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json
RUN rustup target add wasm32-unknown-unknown
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo build --release

FROM ghcr.io/xu-cheng/texlive-debian AS cv

WORKDIR /doc
COPY ./tex ./tex
WORKDIR /doc/tex
RUN latexmk -lualatex -file-line-error -halt-on-error -interaction=nonstopmode cv.tex \
    && test -f cv.pdf

FROM debian:trixie

RUN apt-get update && apt-get install --no-install-recommends --assume-yes ca-certificates libssl3t64 \
    && rm -rf /var/cache/apt/archives /var/lib/apt/lists/* \
    && mkdir -p /usr/share/kruhlmann-dev
COPY static/ /usr/share/kruhlmann-dev/static
COPY --from=cv /doc/tex/include/tex /usr/share/kruhlmann-dev/tex/include/tex
COPY --from=cv /doc/tex/cv.pdf /usr/share/kruhlmann-dev/static/doc/cv.pdf
COPY --from=builder /app/static/wasm/ /usr/share/kruhlmann-dev/static/wasm
COPY --from=builder /app/target/release/kruhlmann-dev /usr/local/bin/kruhlmann-dev

ENV STATIC_DIR="/usr/share/kruhlmann-dev/static"
ENV TEX_DIR="/usr/share/kruhlmann-dev/tex"

ENTRYPOINT ["/usr/local/bin/kruhlmann-dev"]
