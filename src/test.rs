use crate::{json, Response, StatusCode};

#[test]
fn json_response() {
    let mut default_response = Response::default();
    let response = default_response
        .status(StatusCode::Ok)
        .body_json(json::json!({"message": "Hello, World!"}));

    assert_eq!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 27\r\n\r\n{\"message\":\"Hello, World!\"}",
        response.to_string()
    );
}
