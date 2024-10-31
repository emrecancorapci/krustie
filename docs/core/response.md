Represents the response object that will be sent to the client

In Krustie, reponse objects doesn't need to be returned and created. Instead they need to be modified using functions.

The basic functions to create a response are: `status`, `headers`, `body` and `body_json`.

- `status` *sets* the status code of the response. It takes [StatusCode] as an argument.
- `headers` *extends* the current headers of the response. It takes a `HashMap<string, string>` as an argument.
  - If you want to add just *a single header* there is the `set_header` function which takes two strings, a key and a value, as an argument.
- `body`

Because they are mutable, they don't need to be returned from the controllers.

Response can be basicaly built by using the `status`, `headers` and `body` functions which can be
chained like this:

```rust
use krustie::{ Response, StatusCode, Request, response::ContentType, };
use std::collections::HashMap;

fn get(request: &Request, response: &mut Response) {
  let mut headers = HashMap::new();

  headers.insert("Server".to_string(), "Krustie".to_string());

  response
    .status(StatusCode::Ok)
    .headers(headers)
    .body(b"Hello, World!".to_vec(), ContentType::Text);
}
```

But there are other functions such as `set_header` and `set_body` can be useful especially when creating a middleware.
