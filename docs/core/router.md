Represents a router that can be used to create a tree of routes

# Routing

The client sends a request to the server with a specific path such as `/`, `/users` or`/users/24` and a method such as  `GET` or `POST`. The server then needs to determine which handler function to call based on the path and method of the request. This is where routing comes into play.

Krustie provides the perfect router that allows you to define routes and associate them with handler functions. The router supports parameter and query string parsing, making it easy to extract data from the request.

## Basic Usage

To create a new router, use the [Router::new()] function:

```rust
use krustie::{Server, Router, StatusCode};

let mut server = Server::create();
let mut main_router = Router::new();
let mut user_router = Router::new();

// Define a route for the user router
user_router.get("/", |req, res| {
    res.status(StatusCode::Ok).body_text("Hello from user router!");
});

// Use the user router as a sub-router of the main router
main_router.use_router("/user", user_router);

// Use the main router as the handler for the server
server.use_handler(main_router);
```

## Defining Routes

Define routes using the `get`, `post`, `put`, `delete`, `patch` methods:

```rust
# use krustie::{Router, StatusCode};
# let mut router = Router::new();
#
router.get("/", |req, res| {
    res.status(StatusCode::Ok).body_text("Hello World!");
});

router.post("/users", |req, res| {
    res.status(StatusCode::Ok).body_text("User created!");
});
```

The first argument for the http route methods is the path of the route, and the second argument is a handler function that will be called when the route is matched.

Handler functions can be defined outside of the router definition:

```rust
# use krustie::{Router, Request, Response, StatusCode};
# let mut router = Router::new();
#
fn hello_world(req: &Request, res: &mut Response) {
    res.status(StatusCode::Ok).body_text("Hello World!");
}

fn create_user(req: &Request, res: &mut Response) {
    res.status(StatusCode::Ok).body_text("User created!");
}

router.get("/", hello_world);
router.post("/users", create_user);
```

Route methods can be *chained* together:

```rust
# use krustie::{Router, Request, Response, StatusCode};
# let mut router = Router::new();
#
router.get("/", |req, res| {
        res.status(StatusCode::Ok).body_text("Hello World!");
    })
    .patch("/users", |req, res| {
        res.status(StatusCode::Ok).body_text("User updated!");
    })
    .get("/users/:id", |req, res| {
        let id = req.get_param("id").unwrap();

        res.status(StatusCode::Ok)
            .body_text(format!("User ID: {}", id).as_str());
    });
```

## Route Parameters

Routes can contain parameters that are extracted from the path of the **[Request]**.

Parameters are defined using a colon `:` followed by the parameter name in the route path such as `/:id`, `/:user_id`. You can access the parameter values using the [Request::get_query_param] or [Request::get_param] method of the **[Request]** object:

```rust
# use krustie::{Router, StatusCode};
# let mut router = Router::new();
# let mut user_router = Router::new();
#
user_router.get("/:id", |req, res| {
    let id = req.get_param("id").unwrap();

    res.body_text(format!("User ID: {}", id).as_str());
});

router.use_router("/user", user_router);
```

### For `/hello/:name` Route

| Path | Function | Returns |
| -- | -- | -- |
| `/hello/marvin` | `get_param("name")` | `Some("marvin")` |
| `/hello/mars` | `get_param("planet")` | `None` |
| `/hello/jupiter` | `get_params()` | `[{"name": "jupiter"}]` |

### For `/hello/:name/:planet` Route

| Path | Function | Returns |
| -- | -- | -- |
| `/hello/marvin/earth` | `get_param("planet")` | `Some("earth")` |
| `/hello/spike/ganymede` | `get_param("name")` | `Some("spike")` |
| `/hello/marvin/arrakis` | `get_param("year")` | `None` |
| `/hello/marvin/earth` | `get_params()` | `[{"name": "marvin"}, {"planet": "earth"}]` |

## Query String Parameters

Routes can also contain query string parameters that are extracted from the query string of the **[Request]**.

You don't need to define the query string parameters in the route path. You can access them using the [Request::get_query_params] and [Request::get_query_param] methods of the **[Request]** object.

```rust
# use krustie::{Router, StatusCode};
# let mut router = Router::new();
#
router.get("/users", |req, res| {
    let queries = req.get_query_params();
    let default_str = String::from("Unknown");
    let name = req.get_query_param("name").unwrap_or(&default_str);

    res.status(StatusCode::Ok).body_text(format!("Hello, {}!", name).as_str());
});
```

| Path | Function | Returns |
| -- | -- | -- |
| `/hello?planet=earth` | `get_query_param("planet")` | `Some("earth")` |
| `/hello?planet=earth` | `get_query_param("moon")` | `None` |
| `/hello?planet=earth&moon=luna` | `get_query_param("moon")` | `Some("luna")` |

## Creating Sub-Routers

You can create a router as a sub-router of another router by calling the [Router::use_router] method:

```rust
# use krustie::{Router, StatusCode};
# let mut router = Router::new();
#
let mut user_router = Router::new();
let mut post_router = Router::new();

user_router.get("/", |req, res| {
    res.status(StatusCode::Ok).body_text("Hello from user router!");
});

post_router.get("/", |req, res| {
    res.status(StatusCode::Ok).body_text("Hello from post router!");
});

router.use_router("/user", user_router);
router.use_router("/post", post_router);
```

In the example above, the `user_router` and `post_router` are sub-routers of the `main_router`. The `main_router` will handle requests to `/user` and `/post` paths and delegate the handling of the requests to the respective sub-routers.
