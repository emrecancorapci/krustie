Represents a router that can be used to create a tree of routes

## Example

```rust
use krustie::{ Router, StatusCode };

let mut main_router = Router::new();
let mut user_router = Router::new();
let mut user_id_router = Router::new();

user_id_router
  .get("/", |req, res| {
    res.status(StatusCode::Ok);
  })
  .post("/", |req, res| {
    res.status(StatusCode::Ok);
  });

user_router.use_router("/:id", user_id_router);

let mut deeper_router = Router::new();

main_router.use_router("/admin/user", deeper_router);

main_router.use_router("/user", user_router);
```
