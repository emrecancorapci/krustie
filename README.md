# Krustie - A Basic Backend Framework

[<img alt="github" src="https://img.shields.io/badge/github-krustie-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/emrecancorapci/krustie)
[<img alt="crates.io" src="https://img.shields.io/crates/v/krustie.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/krustie)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-krustie-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/krustie)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/emrecancorapci/krustie/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/emrecancorapci/krustie/actions?query=branch%3main)

Krustie is a simple backend framework. It is designed to be a easy-to-use HTTP web server that can be used for a variety of purposes. Krustie's error-proof design aims to make it harder to write bugs.

> Krustie is still in the early stages of development and ***is not yet ready for production use***. The API is subject to change and there may be bugs or missing features.

## Features

- Router with support for parameter and query string parsing
- Middleware support for routers and endpoints
- Minimal testing support
- JSON data parsing and serialization (using the `serde` library)

### Built-in Middlewares

- Static file serving
- Rate limiting
- Gzip compression

## Start your server

```rust
use krustie::{ Router, Server, StatusCode };

fn main() {
    let mut server = Server::create();
    let mut router = Router::new();

    router.get("/", |_, res| {
        res.status(StatusCode::Ok).body_text("Hello World!");
    });

    server.use_handler(router);

    server.listen(8080);
}
```

## Contributing

All contributions are welcomed. Please open an issue or a pull request to report a bug or request a feature.
