Represents the response object that will be sent to the client

In Krustie, reponse objects doesn't need to be returned and created. Instead they need to be modified using methods.

The basic functions to create a response are: `status`, `headers`, `body` and `body_json`.

- `status` *sets* the status code of the response. It takes [StatusCode] as an argument.
- `headers` *extends* the current headers of the response. It takes a `HashMap<string, string>` as an argument.
  - If you want to add just *a single header* there is the [Response::set_header] function which takes two strings, a key and a value, as an argument.
- `body` *sets* the body of the response. It takes a `Vec<u8>` and a [ContentType] as arguments.
- `body_json` *sets* the body of the response as a JSON object. It takes a `serde_json::Value` as an argument.

Response can be basicaly built by using the `status`, `headers` and `body` functions which can be chained like this:

```rust
# use krustie::{ Response, StatusCode, Request, response::ContentType, };
# use std::collections::HashMap;

fn get(request: &Request, response: &mut Response) {
  let mut headers = HashMap::new();

  headers.insert("Server".to_string(), "Krustie".to_string());

  response
    .status(StatusCode::Ok)
    .headers(headers)
    .body(b"Hello, World!".to_vec(), ContentType::Text);
}
```

But there are other functions such as [Response::set_header] and [Response::set_body] can be useful especially when creating a middleware.
