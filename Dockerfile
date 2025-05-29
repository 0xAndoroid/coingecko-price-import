# Stage 1: Build the binary
# Use the official Rust image to create a build artifact.
# This is a multi-stage build. This stage is named 'builder'.
FROM --platform=linux/arm64/v8 rust:bookworm as builder

ENV RUSTFLAGS='-C target-feature=-crt-static'

# Create a new empty shell project
RUN USER=root cargo new --bin coingecko-price-import
WORKDIR /coingecko-price-import

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm -rf ./src

# Now that the dependency is built, copy your source tree
COPY ./src ./src

RUN rm ./target/release/deps/coingecko_price_import*

# Build for release.
RUN cargo build --locked --release

# Stage 2: Setup the runtime environment
# Use a smaller Debian image to reduce size.
FROM --platform=linux/arm64/v8 debian:bookworm-slim

RUN mkdir /app
WORKDIR /app

RUN apt-get update && apt-get install -y openssl ca-certificates

# Copy the build artifact from the builder stage.
COPY --from=builder /coingecko-price-import/target/release/coingecko-price-import /usr/local/bin/coingecko-price-import
COPY ./Rocket.toml ./Rocket.toml

# Set the default command to run the binary
CMD ["coingecko-price-import"]
