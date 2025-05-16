# Author: FL03 <jo3mccain@icloud.com>

# Latest version (1.86.0) as of May 08, 2024
ARG RUST_VERSION=latest
# ************** STAGE 0 **************
# builder-base: a base image for the build-stage(s)
FROM rust:${RUST_VERSION} AS builder-base
# update and upgrade the system
RUN apt-get update -y && \
    apt-get upgrade -y
# install the required system dependencies
RUN apt-get install -y \
    libssl-dev \
    libsqlite3-dev 
# update any toolchains
RUN rustup update
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
    cargo build --locked --release --all-features --workspace
# ************** STAGE 2 **************
# production-base: A slim base image capable of running the application
FROM debian:bookworm-slim AS runner-base
# update and upgrade the system
RUN apt-get update -y && \
    apt-get upgrade -y
# install the required system dependencies
RUN apt-get install -y \
    libsqlite3-dev
# create a user and group
RUN groupadd -g 10001 agroup && \
    useradd -m -u 10001 -g agroup auser
# switch to the user
USER auser

# copy the binary to the system
COPY --from=builder --chown=auser:agroup /app/target/release/vaulted /usr/local/bin/vaulted
# copy the configuration files
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/.config /opt/vaulted/.config
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/*.config.toml* /opt/vaulted/.config/*.config.toml*
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/Proton.toml* /opt/vaulted/.config/Vaulted.toml*
# set the permissions
RUN chmod +x /usr/local/bin/vaulted && \
    chmod +x /opt/vaulted/.config && \
    chown auser /usr/local/bin/vaulted && \
    chown -R auser /opt/vaulted
# ************** STAGE 3 **************
# production: use the production base image to run the application
FROM runner-base AS runner
# Set the environment variables
ENV APP_CONFIG_DIR="/opt/vaulted/.config" \
    APP_MODE=release \
    APP_HOST="0.0.0.0" \
    APP_PORT=8080 \
    RUST_LOG="vaulted=debug,info,trace"
# set the working directory
WORKDIR /opt/vaulted
# expose the port
EXPOSE ${HOSTPORT}
# set the entrypoint
ENTRYPOINT ["vaulted"]