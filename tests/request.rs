use std::net::SocketAddr;
use krustie::http::core::{HttpMethod, HttpRequest};
use krustie::krustie::request::Request;

#[test]
fn try_from_valid_http_methods() {
    assert_eq!(HttpMethod::try_from("GET").unwrap(), HttpMethod::GET);
    assert_eq!(HttpMethod::try_from("post").unwrap(), HttpMethod::POST);
    assert_eq!(HttpMethod::try_from("DeLeTe").unwrap(), HttpMethod::DELETE);
}

#[test]
fn try_from_invalid_http_method() {
    assert!(HttpMethod::try_from("INVALID").is_err());
}

#[tokio::test]
async fn request_parses_valid_http_request() {
    let raw_request = b"GET /path?key=value HTTP/1.1\r\nHost: example.com\r\nUser-Agent: TestAgent\r\n\r\n";
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let request = Request::new(raw_request, addr).await.unwrap();
    
    assert_eq!(request.get_method(), &HttpMethod::GET);
    assert_eq!(request.get_path(), "/path");
    assert_eq!(request.get_version(), "HTTP/1.1");
    assert_eq!(request.get_header("host"), Some("example.com"));
    assert_eq!(request.get_user_agent(), Some("TestAgent"));
    assert_eq!(request.get_query_param("key"), Some("value"));
}

#[tokio::test]
async fn request_handles_invalid_request_line() {
    let raw_request = b"INVALID REQUEST LINE";
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    assert!(Request::new(raw_request, addr).await.is_err());
}

#[tokio::test]
async fn request_handles_missing_http_version() {
    let raw_request = b"GET /path HTTP/2.0\r\n\r\n";
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    assert!(Request::new(raw_request, addr).await.is_err());
}

#[tokio::test]
async fn request_parses_body_correctly() {
    let raw_request = b"POST /data HTTP/1.1\r\nContent-Length: 11\r\n\r\nHello World";
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let request = Request::new(raw_request, addr).await.unwrap();
    
    assert_eq!(request.get_body(), b"Hello World");
}

#[tokio::test]
async fn request_fails_on_invalid_content_length() {
    let raw_request = b"POST /data HTTP/1.1\r\nContent-Length: ABC\r\n\r\nHello World";
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    assert!(Request::new(raw_request, addr).await.is_err());
}

#[tokio::test]
async fn request_fails_when_body_is_empty_but_content_length_nonzero() {
    let raw_request = b"POST /data HTTP/1.1\r\nContent-Length: 5\r\n\r\n";
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    assert!(Request::new(raw_request, addr).await.is_err());
}