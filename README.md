# Krustie - A Basic Backend Framework

[<img alt="github" src="https://img.shields.io/badge/github-krustie-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/emrecancorapci/krustie)
[<img alt="crates.io" src="https://img.shields.io/crates/v/krustie.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/krustie)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-krustie-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/krustie)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/emrecancorapci/krustie/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/emrecancorapci/krustie/actions?query=branch%3main)

Krustie is a backend framework written in Rust. It is currently a work in progress and not yet ready for production use. This project serves as a personal learning experience, and contributions or feedback are welcome.

## Features

- Basic request and response handling
- Stacked Router
- Server Middleware support
- Router Middleware support
- Static file serving
- Gzip Compression (Thanks to [flate2](https://crates.io/crates/flate2))
- JSON parsing (Thanks to [serde_json](https://crates.io/crates/serde_json))

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/)
- [Cargo](https://doc.rust-lang.org/cargo/)

### Installation

To add Krustie to your project, include it in your `Cargo.toml`:

```toml
[dependencies]
krustie = "0.1.5"
```

Alternatively, you can add it using Cargo:

```bash
cargo add krustie
```

2. Start your server:

```rust
use krustie::{
  server::Server,
  router::{ Router, methods::Endpoints },
  response::{ HttpResponse, StatusCode },
  request::HttpRequest,
  middleware::{ MiddlewareHandler, Middleware, gzip::Gzip },
  json::{ json, get_string_from_json },
};
use std::collections::HashMap;
use std::net::Ipv4Addr;

struct AddKrustieHeader;

impl Middleware for AddKrustieHeader {
  fn middleware(req: &HttpRequest, res: &mut HttpResponse) {
    res.insert_header("Server", "Krustie");
  }
}

fn main() {
  let mut server = Server::create();
  let mut router = Router::new();
  let mut sub_router = Router::new();

  sub_router
    .get(|_, res| {
      let body = json!({"message": "Hello, World!"});
      res.status(StatusCode::Ok).json_body(body);
    })
    .post(post_req);

  router.use_router("home", sub_router);

  server.use_handler(router);
  server.use_handler(AddKrustieHeader);
  server.use_handler(Gzip);

  server.listen((127, 0, 0, 1), 8080);
}

fn post_req(req: &HttpRequest, res: &mut HttpResponse) {
  match req.get_body_as_json() {
    Ok(body) => {
      if get_string_from_json(body.get("server")).unwrap() == "Krustie" {
        res.status(StatusCode::Ok).json_body(body);
      } else {
        res.status(StatusCode::try_from(201).unwrap()).json_body(json!({"error": "Invalid server"}));
      }
    }
    Err(_) => {
      res.status(StatusCode::BadRequest).json_body(json!({"error": "Invalid JSON"}));
    }
  }
}
```

3. Run your server:

```bash
cargo run
```

## Contributing

As an inexperienced developer contributions will be welcomed. Please open an issue or a pull request.

