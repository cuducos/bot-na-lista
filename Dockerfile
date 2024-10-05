FROM rust:1-slim-bookworm AS build

WORKDIR /usr/src/bot-na-lista
ENV BUILD_PKGS="build-essential ca-certificates g++ libpq-dev libssl-dev pkg-config"


COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
RUN apt-get clean && \
    apt-get update && \
    apt-get install -y ${BUILD_PKGS} && \
    cargo install --path . && \
    cargo clean && \
    apt-get purge -y ${BUILD_PKGS} && \
    apt-get -y autoremove && \
    rm -rf /var/lib/apt/lists/*

FROM debian:bookworm-slim

RUN apt-get clean && \
    apt-get update && \
    apt-get install -y ca-certificates libpq-dev libssl-dev && \
    apt-get -y autoremove && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/local/cargo/bin/bot-na-lista* /usr/local/bin/

CMD ["bot-na-lista"]
