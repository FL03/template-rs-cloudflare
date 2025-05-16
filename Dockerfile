# Author: FL03 <jo3mccain@icloud.com>

# Latest version (1.86.0) as of May 08, 2024
ARG RUST_VERSION=latest
# ************** STAGE 0 **************
# builder-base: a base image for the build-stage(s)
FROM rust:${RUST_VERSION} AS builder-base
# update and upgrade the system
RUN apt-get update -y && \
    apt-get upgrade -y
# install the required system packages
RUN apt-get install -y \
    clang \
    cmake \
    libopenblas-dev \
    libssl-dev \
    libsqlite3-dev \
    nodejs \
    npm \
    pkg-config
# update any toolchains
RUN rustup update
# install the required system dependencies
RUN cargo install \
    cargo-binstall
# install additional tooling
RUN cargo binstall wasm-pack worker-build -y
# ************** STAGE 1 **************
# builder: build the project using the builder-base image
FROM builder-base AS builder
# setup the working directory
WORKDIR /app
# copy the source code
ADD . .
# build the project
RUN --mount=type=cache,target=/workspace/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release --all-features --workspace --target wasm32-unknown-unknown
# ************** STAGE 2 **************
# production-base: A slim base image capable of running the application
FROM scratch AS runner-base
#
# copy the binary to the system
COPY --from=builder --chown=auser:agroup /app/target/wasm32-unknown-unknown/release/rscloud /opt/rscloud
# copy the configuration files
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/.config /opt/rscloud/.config
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/*.config.toml* /opt/rscloud/.config/*.config.toml*
# ************** STAGE 3 **************
# production: use the production base image to run the application
FROM runner-base AS runner
# Set the environment variables
ENV APP_CONFIG_DIR="/opt/rscloud/.config" \
    APP_MODE=release \
    APP_HOST="0.0.0.0" \
    APP_PORT=8080 \
    RUST_LOG="rscloud=debug,info,trace"
# set the working directory
WORKDIR /opt/rscloud
# expose the port
EXPOSE ${APP_PORT}
# set the entrypoint
ENTRYPOINT ["rscloud"]