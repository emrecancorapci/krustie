Represents the incoming HTTP request

It contains information about the request, such as the request `method`, `path`, `query string`, `parameters`, `headers`, and `body`.

[`Request`] cannot be accessed directly or modified anywhere in the code. It can only be accessed through the handler or middleware function with an immutable reference via methods.

## All Methods

| Method                       | Return Type                | Description                                                       |
|------------------------------|----------------------------|-------------------------------------------------------------------|
| `get_body()`                 | `RequestBody`              | Returns the body of the request.                                  |
| `get_header(key: &str)`      | `Option<&str>`             | Returns the value of the specified header.                        |
| `get_headers()`              | `&HashMap<String, String>` | Returns a reference to the request headers.                       |
| `get_method()`               | `&HttpMethod`              | Returns the request method.                                       |
| `get_param(key: &str)`       | `Option<&str>`             | Returns the value of the parameter with the specified name.       |
| `get_params()`               | `&HashMap<String, String>` | Returns a map of all parameters.                                  |
| `get_path_array()`           | `Vec<String>`              | Returns the request path as an array of segments.                 |
| `get_path()`                 | `&str`                     | Returns the request path.                                         |
| `get_peer_addr()`            | `&SocketAddr`              | Returns the address of the peer that sent the request.            |
| `get_query_param(key: &str)` | `Option<&str>`             | Returns the value of the query parameter with the specified name. |
| `get_query_params()`         | `HashMap<String, String>`  | Returns a map of all query parameters.                            |
