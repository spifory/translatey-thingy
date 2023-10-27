FROM rust:1.72.1-slim-bookworm

WORKDIR /bot

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt update && apt install -y libssl-dev pkg-config

COPY ./ ./

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/target \
    cargo build --release --verbose && \
    cp ./target/release/translatey-thingy ./ && rm src/*.rs

CMD [ "./translatey-thingy" ]
