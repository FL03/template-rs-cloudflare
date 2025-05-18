---
title: Quickstart
description: A detailed guide on how to get started with the template-rs-cloudflare project.
---

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/)
- [Rust](https://www.rust-lang.org/tools/install)

### _Utilities_

- [cargo-binstall](https://github.com/cargo-bins/cargo-binstall): streamline the installation of any additional tools needed

## Getting Started

Ensure that `rustup` and all installed toolchains are updated:

```bash
rustup update
```

before installing the `wasm32-unknown-unknown` target:

```bash
rustup target add wasm32-unknown-unknown --toolchain stable
```

### Building from the source

Get started by cloning the repository

```bash
git clone https://github.com/FL03/template-rs-cloudflare.git --branch main
```

Then, navigate to the project directory:

```bash
cd template-rs-cloudflare
```

#### Native

For native development, you can run the server using cargo:

```bash
cargo run --locked --release --features native
```

#### WebAssembly

Build the project using the wasm32 target:

```bash
cargo build --locked --workspace --release --features web --target wasm32-unknown-unknown
```

##### _Cloudflare_

Install the `worker-build` utility using `cargo-binstall:

```bash
cargo binstall -y worker-build
```

Finally, build the project using the `worker-build` tool:

```bash
worker-build build -d build --release --features web
```

This will create a `build` directory containing the compiled WASM files and the necessary JavaScript glue code.

or, use `npx` and `wrangler` to build the project:

```bash
npx wrangler@latest build
```

##### _wasm-pack_

Alternatively, you can use the `wasm-pack` tool to build the project. Start by installing the utility using `cargo-binstall:

```bash
cargo binstall -y worker-build
```

Then, build the project using the `wasm-pack` tool:

```bash
wasm-pack build --target web --release --out-dir build
```

#### Docker

You can also build the project using Docker. Start by building the Docker image:

```bash
docker buildx build --platform linux/amd64 -t jo3mccain/template-rs-cloudflare:latest -f .docker/native.dockerfile .
```

Then, run the Docker container:

```bash
docker run -d -it --rm -p 8080:8080 -v $(pwd):/app jo3mccain/template-rs-cloudflare:latest
```

This will start the server and bind it to port 8080 on your host machine. You can access the server by navigating to `http://localhost:8080` in your web browser.
