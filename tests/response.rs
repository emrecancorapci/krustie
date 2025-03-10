use std::collections::HashMap;

use krustie::{
    http::core::HttpResponse,
    krustie::{response::Response, status_code::StatusCode},
};

#[test]
fn serialize_response() {
    let mut default_response = Response::new();
    let mut headers = HashMap::new();

    headers.insert(String::from("Hello"), String::from("World"));
    headers.insert(String::from("Meaning of the life"), String::from("42"));

    let response = default_response
        .set_status(StatusCode::Ok)
        .set_header("Server", "Krustie")
        .extend_headers(headers)
        .set_body_text("Hello world!");

    assert_eq!(
        "HTTP/1.1 200 OK\r\nContent-Length: 12\r\nContent-Type: text/plain\r\nHello: World\r\nMeaning of the life: 42\r\nServer: Krustie\r\n\r\nHello world!",
        response.to_string()
    );
}
