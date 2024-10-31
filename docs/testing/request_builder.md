A builder for the [`Request`] struct

This builder is used to create a [`Request`] struct to be used in testing.

# Example

```rust
use krustie::{HttpMethod, Request, request::RequestBody};

let request = Request::builder()
  .method(HttpMethod::GET)
  .path("/echo/hello")
  .header("Content-Type", "application/json")
  .body(RequestBody::Text("Hello, World!".to_string()))
  .build();

assert_eq!(request.get_method(), &HttpMethod::GET);
assert_eq!(request.get_path(), "/echo/hello");
assert_eq!(request.get_header("Content-Type"), Some("application/json"));
assert_eq!(request.get_body(), &RequestBody::Text("Hello, World!".to_string()));
```

Go to [`Request`]
