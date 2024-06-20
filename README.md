# Krustie - A Basic Backend Framework

Krustie is a backend written in Rust. It is a work in progress and is not yet ready for use.

It is a hobby project and is intended to be a learning experience for me. I am not a professional developer and I am learning as I go. I am open to feedback and suggestions.

## Features

- [x] Basic request and response handling
- [x] Stacked Router
- [x] Server Middleware support
- [x] Router Middleware support
- [x] Static file serving
- [x] Compression (gzip)
- [x] JSON parsing (Thanks to [serde_json](https://github.com/serde-rs/json))

## Getting Started

### Prerequisites

- Rust
- Cargo

### Installation

1. Add Krustie to your `Cargo.toml`:

```toml
[dependencies]
krustie = "0.1.5"
```

or use `cargo add` in your terminal:

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
  let mut server = Server::create(Ipv4Addr::new(127, 0, 0, 1), 8080);
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

## Planned Features

### Basic Server Features

- [ ] Database support
- [ ] Authentication
- [ ] Authorization
- [ ] Error handling
- [ ] File uploads
- [ ] Websockets

### Basic Web Server Features

- [ ] Templating
- [ ] Session management

### Basic API Server Features

- [ ] XML parsing
- [ ] Query parameter parsing
- [ ] Request validation
- [ ] Response formatting

### Security

- [ ] Rate limiting
- [ ] CSRF protection
- [ ] XSS protection
- [ ] SQL injection protection
- [ ] Secure headers
- [ ] Content Security Policy

### Performance

- [ ] Caching
- [ ] Connection pooling
- [ ] Load balancing
- [ ] Clustering

### Monitoring

- [ ] Metrics
- [ ] Tracing
- [ ] Logging

### Documentation

- [ ] API documentation
- [ ] Code documentation
- [ ] Examples
