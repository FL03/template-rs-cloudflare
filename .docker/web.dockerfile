# Author: FL03 <jo3mccain@icloud.com>

# Latest version (1.86.0) as of May 08, 2024
ARG RUST_VERSION=latest
# ************** STAGE 0 **************
# builder-base: a base image for the build-stage(s)
FROM rust:${RUST_VERSION} AS builder-base
# update and upgrade the system
RUN apt-get update -y && \
    apt-get upgrade -y
# add additional targets for rustup
RUN rustup target add wasm32-unknown-unknown
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
RUN cargo build --locked --release --target wasm32-unknown-unknown -p template-rs-cloudflare --lib --features web
# ************** STAGE 2 **************
# production-base: use the scratch image to run the application
FROM scratch AS prod-base
# copy the binary to the system
COPY --from=builder /app/target/wasm32-unknown-unknown/release/rscloud /app
# copy the configuration files
COPY --from=builder --link /app/.config /app/.config
COPY --from=builder --link /app/*.config.toml* /app/.config/*.config.toml*
# ************** STAGE 3 **************
# production: Final image to run the application
FROM prod-base AS prod
# switch the working directory
WORKDIR /app
# declare some environment variables
ENV APP_CONFIG_DIR="/app/.config" \
    APP_MODE="release" \
    APP_HOST="0.0.0.0" \
    APP_PORT=8080 \
    RUST_LOG="rscloud=debug,info,trace"
# expose the port
EXPOSE ${APP_PORT}
# set the entrypoint
ENTRYPOINT ["rscloud"]