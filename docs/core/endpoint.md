Holds the route data and the controller function

# Example

```rust
use krustie::{ Server, Router, Endpoint, HttpMethod, StatusCode, Request, Response };

fn get(req: &Request, res: &mut Response) {
  res.status(StatusCode::Ok).body_text("Hello, World!");
}

# let mut router = Router::new();
let endpoint = Endpoint::new(HttpMethod::GET, get);

router.use_endpoint("/hello", endpoint);

```
