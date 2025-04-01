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


#[test]
fn serialize_empty_response() {
    let response = Response::new();

    assert_eq!(
        "HTTP/1.1 200 OK\r\nContent-Length: 0\r\nContent-Type: text/plain\r\n\r\n",
        response.to_string()
    );
}

#[test]
fn serialize_response_with_custom_status() {
    let mut response = Response::new();

    let response = response.set_status(StatusCode::NotFound);

    assert_eq!(
        "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nContent-Type: text/plain\r\n\r\n",
        response.to_string()
    );
}

#[test]
fn serialize_response_with_multiple_headers() {
    let mut headers = HashMap::new();
    let mut response = Response::new();

    headers.insert(String::from("X-Custom-Header"), String::from("CustomValue"));
    headers.insert(String::from("Cache-Control"), String::from("no-cache"));

    let response = response
        .set_status(StatusCode::Ok)
        .extend_headers(headers);

    assert!(response.to_string().contains("X-Custom-Header: CustomValue"));
    assert!(response.to_string().contains("Cache-Control: no-cache"));
}

#[test]
fn serialize_response_with_body() {
    let mut response = Response::new();

    let response = response
        .set_status(StatusCode::Ok)
        .set_body_text("This is a test body");

    assert!(response.to_string().contains("This is a test body"));
}

#[test]
fn serialize_response_with_different_content_type() {
    let mut response = Response::new();

    let response = response
        .set_status(StatusCode::Ok)
        .set_header("Content-Type", "application/json")
        .set_body_text("{\"message\": \"Hello\"}");

    assert!(response.to_string().contains("Content-Type: application/json"));
    assert!(response.to_string().contains("{\"message\": \"Hello\"}"));
}

#[test]
fn serialize_response_full_assert() {
    let mut response = Response::new();

    let response = response
        .set_status(StatusCode::Ok)
        .set_header("Server", "Krustie")
        .set_body_text("Full response test");
    
    let expected_response = "HTTP/1.1 200 OK\r\nContent-Length: 18\r\nContent-Type: text/plain\r\nServer: Krustie\r\n\r\nFull response test";
    
    assert_eq!(expected_response, response.to_string());
}