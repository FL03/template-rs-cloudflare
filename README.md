# template-rs-cloudflare

[![Docker Image Version](https://img.shields.io/docker/v/jo3mccain/template-rs-cloudflare?sort=semver&style=for-the-badge&logo=docker)](https://hub.docker.com/r/jo3mccain/template-rs-cloudflare)

[![deploy](https://github.com/FL03/template-rs-cloudflare/actions/workflows/deploy.yml/badge.svg)](https://github.com/FL03/template-rs-cloudflare/actions/workflows/deploy.yml)
[![rust](https://github.com/FL03/template-rs-cloudflare/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/template-rs-cloudflare/actions/workflows/rust.yml)

***

_**The application is currently in the early stages of development and is not yet ready for production use.**_

A server optimized for WASM applications

## Features

- [x] Feature 1

## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/template-rs-cloudflare.git
cd template-rs-cloudflare
```

#### _Building the project_

```bash
cargo build --workspace --release -F full,cf --target wasm32-unknown-unknown
```

#### _Running tests_

```bash
cargo test --workspace --release -F full,cf --target wasm32-unknown-unknown
```

## Usage

### Pre-requisites

For convenience, we recommend using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) to install rust binaries. You can install it with the following command:

```bash
cargo install cargo-binstall
```

Once you have `cargo-binstall` installed, you can install the required packages with the following command:

```bash
cargo binstall -y wasm-pack worker-build
```

### Running the server

```bash
pzzld serve --port 8080
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
