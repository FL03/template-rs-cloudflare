---
title: template-rs-cloudflare
description: A RESTful application template built for Cloudflare Workers using Rust and WebAssembly.
---

[![Docker Image Version](https://img.shields.io/docker/v/jo3mccain/template-rs-cloudflare?sort=semver&style=for-the-badge&logo=docker)](https://hub.docker.com/r/jo3mccain/template-rs-cloudflare)

[![deploy](https://github.com/FL03/template-rs-cloudflare/actions/workflows/deploy.yml/badge.svg)](https://github.com/FL03/template-rs-cloudflare/actions/workflows/deploy.yml)
[![rust](https://github.com/FL03/template-rs-cloudflare/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/template-rs-cloudflare/actions/workflows/rust.yml)

***

_**The application is currently in the early stages of development and is not yet ready for production use.**_

A server optimized for WASM applications

## Features

- [x] Feature 1

## Getting Started

Make sure you have the following installed:

- [Docker](https://docs.docker.com/get-docker/)
- [Rust](https://www.rust-lang.org/tools/install)

Make sure you have the `cargo-binstall` utility installed to streamline the installation of any additional tools needed.

```bash
cargo install cargo-binstall
```

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/template-rs-cloudflare.git --branch main
```

Then, navigate to the project directory:

```bash
cd template-rs-cloudflare
```

#### Native

Finally, we can run the server using cargo:

```bash
cargo run --bin rscloud --features native
```

#### WebAssembly

Build the project using the wasm32 target:

```bash
cargo build --workspace --release --features web --target wasm32-unknown-unknown
```

#### Cloudflare

Install the `wasm-pack` and `worker-build` tools using `cargo-binstall:

```bash
cargo binstall -y wasm-pack worker-build
```

Finally, build the project using the `worker-build` tool:

```bash
worker-build build -d build --release --features web
```

This will create a `build` directory containing the compiled WASM files and the necessary JavaScript glue code.

Alternatively, you can use the `wasm-pack` tool to build the project:

```bash
wasm-pack build --target web --release --out-dir build
```

or, use `npx` and `wrangler` to build the project:

```bash
npx wrangler@latest build
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
