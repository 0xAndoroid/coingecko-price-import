FROM --platform=linux/arm64/v8 rust:bookworm as builder

ENV RUSTFLAGS='-C target-feature=-crt-static'

RUN USER=root cargo new --bin coingecko-price-import
WORKDIR /coingecko-price-import

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm -rf ./src

COPY ./src ./src

RUN rm ./target/release/deps/coingecko_price_import*

RUN cargo build --locked --release

FROM --platform=linux/arm64/v8 debian:bookworm-slim

RUN mkdir /app
WORKDIR /app

RUN apt-get update && apt-get install -y openssl ca-certificates

COPY --from=builder /coingecko-price-import/target/release/coingecko-price-import /usr/local/bin/coingecko-price-import
COPY ./Rocket.toml ./Rocket.toml

CMD ["coingecko-price-import"]
