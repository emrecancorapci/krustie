Trait to be implemented for creating middleware

It can be executed before or after the request is handled depends on order of the middlewares and routers.

To create a middleware, implement the [`Middleware`] trait. Which has a single function called [`Middleware::middleware()`].

It takes itself as `&self`, a [`Request`] and a `HandlerResult`.

If there is no property declared in the struct, struct can be used directly. Or it can be used as a value if it needs to be initialized.

## Example

- In this example `AddKrustieHeader` can be used without initialized: `server.add_handler(AddKrustieHeader)`

```rust
use krustie::{ Request, Response, Middleware, server::route_handler::HandlerResult };

#[derive(Clone)]
struct AddKrustieHeader;

impl AddKrustieHeader {
  fn add_header(res: &mut Response) {
    res.set_header("Server", "Krustie");
  }
}

impl Middleware for AddKrustieHeader {
  fn middleware(&mut self, req: &Request, res: &mut Response) -> HandlerResult {
    AddKrustieHeader::add_header(res);
    HandlerResult::Next
  }
}
```

- In this example `AddHeader` need to be initialized.

```rust
use krustie::{ Request, Response, Middleware, server::route_handler::HandlerResult };

#[derive(Clone)]
struct AddHeader {
    server: String,
    value: String,
}

impl AddHeader {
    fn new(server: &str, value: &str) -> Self {
        Self { server: server.to_string(), value: value.to_string() }
    }
}

impl Middleware for AddHeader {
  fn middleware(&mut self, _: &Request, res: &mut Response) -> HandlerResult {
    res.set_header(&self.server, &self.value);
    HandlerResult::Next
  }
}
```
