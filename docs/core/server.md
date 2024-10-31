Represents the server instance

## A Complete Example

```no_run
use krustie::{
  Server,
  Router,
  Request,
  Response,
  Middleware,
  StatusCode,
  json::{ get_string_from_json, json },
  middleware::{ GzipEncoder, ServeStatic },
  server::route_handler::HandlerResult,
  request::RequestBody,
  response::ContentType,
};

fn main(){
  let mut server = Server::create();
  let mut router = Router::new();

  router.get("/", |_, res| {
    res.status(StatusCode::Ok).body(
      b"<html><body><h1>Hello, World!</h1></body></html>".to_vec(),
      ContentType::Html
    );
  });

  let mut sub_router = Router::new();

  sub_router
    .get("/", |_, res| {
      let body = json!({"message": "Hello, World!"});
      res.status(StatusCode::Ok).body_json(body);
    })
    .post("/", post_req);

  router.use_router("/home", sub_router);

  server.use_handler(router);

  // Middlewares

  let krustie_middleware = AddHeader::new("Server", "Krustie");

  server.use_handler(krustie_middleware);
  server.use_handler(GzipEncoder);
  server.use_handler(ServeStatic::new("public"));

  server.listen(8080);
}

fn post_req(req: &Request, res: &mut Response) {
  match req.get_body() {
    RequestBody::Json(json) => {
      let key_result = json.get("server");
      if get_string_from_json(key_result).unwrap() == "Krustie" {
        res.status(StatusCode::Ok).body_json(json!({"message": "Valid server"}));
      } else {
        res.status(StatusCode::try_from(201).unwrap()).body_json(json!({"error": "Invalid server"}));
      }
    },
    _ => {
      res.status(StatusCode::BadRequest).body_json(json!({"error": "Invalid JSON"}));
    }
  }
}

#[derive(Clone)]
struct AddHeader {
  key: String,
  value: String,
}

impl AddHeader {
  fn new(key: &str, value: &str) -> Self {
    Self { key: key.to_string(), value: value.to_string() }
  }
}

impl Middleware for AddHeader {
  fn middleware(&mut self, _: &Request, res: &mut Response) -> HandlerResult {
    res.set_header(&self.key, &self.value);
    HandlerResult::Next
  }
}
```
