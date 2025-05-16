# Author: FL03 <jo3mccain@icloud.com>

# Latest version (1.86.0) as of May 08, 2024
ARG RUST_VERSION=latest
# ************** STAGE 0 **************
# builder-base: a base image for the build-stage(s)
FROM rust:${RUST_VERSION} AS builder-base
# update and upgrade the system
RUN apt-get update -y && \
    apt-get upgrade -y
# update any toolchains
RUN rustup update
# ************** STAGE 1 **************
# builder: build the project using the builder-base image
FROM builder-base AS builder
# declare some environment variables
ENV RUST_BACKTRACE=1 \
    CARGO_NET_GIT_FETCH_WITH_CLI=true \
    CARGO_NET_GIT_FETCH_WITH_CLI_RETRIES=3 \
    CARGO_NET_GIT_FETCH_WITH_CLI_RETRY_WAIT_SECS=5
# setup the working directory
WORKDIR /app
# copy the source code
ADD . .
# build the project
RUN --mount=type=cache,target=/workspace/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release --all-features --workspace --target wasm32-unknown-unknown
# ************** STAGE 2 **************
# production-base: use the scratch image to run the application
FROM scratch AS prod-base
# copy the binary to the system
COPY --from=builder --chown=auser:agroup /app/target/wasm32-unknown-unknown/release/rscloud .
# copy the configuration files
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/.config /.config
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/*.config.toml* /.config/*.config.toml*
# ************** STAGE 3 **************
# production-base: A slim base image capable of running the application
FROM prod-base AS prod
# declare some environment variables
ENV APP_CONFIG_DIR=".config" \
    APP_MODE="release" \
    APP_HOST="0.0.0.0" \
    APP_PORT=8080 \
    RUST_LOG="rscloud=debug,info,trace"
#
# copy the binary to the system
COPY --from=builder --chown=auser:agroup /app/target/wasm32-unknown-unknown/release/rscloud .
# copy the configuration files
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/.config /.config
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/*.config.toml* /.config/*.config.toml*

# expose the port
EXPOSE ${APP_PORT}
# set the entrypoint
ENTRYPOINT ["rscloud"]