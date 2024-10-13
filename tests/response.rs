use std::collections::HashMap;

use krustie::{json, Response, StatusCode};

#[test]
fn serialize_response() {
    let mut default_response = Response::default();
    let mut headers = HashMap::new();

    headers.insert(String::from("Hello"), String::from("World"));
    headers.insert(String::from("Meaning of the life"), String::from("42"));

    let response = default_response
        .status(StatusCode::Ok)
        .set_header("Server", "Krustie")
        .headers(headers)
        .body_json(json::json!({"message": "Hello, World!"}));

    assert_eq!(
        "HTTP/1.1 200 OK\r\nContent-Length: 27\r\nContent-Type: application/json\r\nHello: World\r\nMeaning of the life: 42\r\nServer: Krustie\r\n\r\n{\"message\":\"Hello, World!\"}",
        response.to_string()
    );
}
