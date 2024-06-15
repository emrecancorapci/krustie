# Krustie - A Basic Backend Framework

Krustie is a backend written in Rust. It is a work in progress and is not yet ready for use.

It is a hobby project and is intended to be a learning experience for me. I am not a professional developer and I am learning as I go. I am open to feedback and suggestions.

## Features

- [x] Basic request and response handling
- [x] Stacked Router
- [x] Server Middleware support (Router Middleware support will be added soon)
- [x] Static file serving

## Getting Started

### Prerequisites

- Rust
- Cargo

### Installation

1. Add Krustie to your `Cargo.toml`:

```toml
[dependencies]
krustie = "0.1.2"
```

2. Start your server:

```rust
use krustie::{ server::Server, router::Router, response::StatusCode };

fn main() {
    match Server::create(4221) {
        Ok(mut server) => {
            let mut router = Router::new();

            router.get("", |req, res| {
                res.status(StatusCode::Ok);
            });
            
            server.use_router(router);
            server.listen()
        }
        Err(err) => { println!("Server cannot created. {}", err) }
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

- [ ] JSON parsing
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
- [ ] Compression
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
