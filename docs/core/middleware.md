Trait to be implemented for creating middleware

# Middleware

```rust
# use krustie::{HandlerResult, Request, Response};
# struct Middleware {};
# impl Middleware {
fn middleware(&mut self, request: &Request, response: &mut Response) -> HandlerResult
# {HandlerResult::Next}
# }
```

Middleware is some kind of handler that is executed before or after the request is processed by the route handler. It is a function that has access to the [Request] and [Response] objects and returns [HandlerResult] which is determines if the request should be processed further or not.

## Creating Middleware

In order to create a middleware, first you need to define a struct that derives the [Clone] trait and implements the [Middleware] trait:

```rust
use krustie::{HandlerResult, Middleware, Request, Response, Router, Server, StatusCode};

#[derive(Clone)]
struct Logger;

impl Middleware for Logger {
    fn middleware(&mut self, request: &Request, response: &mut Response) -> HandlerResult {
        println!(
            "Request received from {}",
            request.get_peer_addr().to_string()
        );
        HandlerResult::Next
    }
}

// Then you can use the middleware as a handler in the server:

fn main() {
    let mut server = Server::create();
    let mut router = Router::new();

    router.get("/", |req, res| {
        res.status(StatusCode::Ok).body_text("Hello World!");
    });

    server.use_handler(Logger);
    server.use_handler(router);
}
```

In the example above, the `Logger` struct implements the [Middleware] trait and the [Middleware::middleware()] method. The method prints the IP address of the client that made the request. The `Logger` struct is then used as a handler in the server.

## Controlling the Flow of Execution

To control the flow of execution, the [Middleware::middleware()] method should return a enum value of [HandlerResult].

- By returning [HandlerResult::Next], the request *will* be processed further by the next middleware or route handler.

- By returning [HandlerResult::End], the request *will not* be processed further and the response will be sent back to the client.

```rust
use krustie::{HandlerResult, Middleware, Request, Response, StatusCode};

#[derive(Clone)]
struct Auth;

impl Middleware for Auth {
    fn middleware(&mut self, request: &Request, response: &mut Response) -> HandlerResult {
        if request.get_headers().get("Authorization").is_none() {
            response
                .status(StatusCode::Unauthorized)
                .body_text("Unauthorized");
            return HandlerResult::End;
        }

        return HandlerResult::Next;
    }
}
```

In the example above, the `Auth` middleware checks if the request has an `Authorization` header. If the header is not present, the middleware sets the status code to `401 Unauthorized` and sends a response back to the client. The [HandlerResult::End] enum value is returned to stop the request from being processed further.
