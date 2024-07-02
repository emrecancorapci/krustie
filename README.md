# Krustie - A Basic Backend Framework

[<img alt="github" src="https://img.shields.io/badge/github-krustie-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/emrecancorapci/krustie)
[<img alt="crates.io" src="https://img.shields.io/crates/v/krustie.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/krustie)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-krustie-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/krustie)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/emrecancorapci/krustie/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/emrecancorapci/krustie/actions?query=branch%3main)

Krustie is a backend framework written in Rust. It is currently a work in progress and not yet ready for production use. This project serves as a personal learning experience, and contributions or feedback are welcome.

## Features

- Basic request and response handling
- Stackable Router
- General Middleware support
- Router Middleware support
- JSON parsing ([serde_json](https://crates.io/crates/serde_json))

### Builtin Middlewares

- Static file serving
- Gzip encoding ([flate2](https://crates.io/crates/flate2))

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/)
- [Cargo](https://doc.rust-lang.org/cargo/)

### Installation

#### Add Krustie to your project

Include it in your `Cargo.toml`:

```toml
[dependencies]
krustie = "0.1.6"
```

Run the following Cargo command in your project directory:

```bash
cargo add krustie
```

#### Start your server

```rust
use krustie::{ Server, Router, StatusCodes };

fn main() {
    let mut server = Server::create();
    let mut router = Router::new();

    router.get(|_, res| {
        res.status(StatusCode::Ok)
            .body(b"Hello World!".to_vec(), "text/plain");
    });

    server.use_handler(router);

    server.listen((127, 0, 0, 1), 8080);
}
```

#### Run your server

```bash
cargo run
```

## Contributing

As an inexperienced developer contributions will be welcomed. Please open an issue or a pull request.
