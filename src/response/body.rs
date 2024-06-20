use super::HttpResponse;
use serde_json::Value as JsonValue;

impl HttpResponse {
    /// Sets the body of the response. Function sets `Content-Length` automatically but needs `Content-Type` to be set manually.
    ///
    /// If `Content-Type` is not set, it defaults to `text/plain`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{response::{HttpResponse, StatusCode}, request::HttpRequest};
    ///
    /// fn get(request: &HttpRequest, response: &mut HttpResponse) {
    ///     response.body(b"Hello, World!".to_vec(), "text/plain");
    /// }
    /// ```
    ///
    /// ```rust
    /// use krustie::{response::{HttpResponse, StatusCode}, request::HttpRequest};
    ///
    /// fn get(request: &HttpRequest, response: &mut HttpResponse) {
    ///    response.status(StatusCode::Ok).body(b"<html><body><h1>Hello, World!</h1></body></html>".to_vec(), "text/html");
    /// }
    /// ```
    pub fn body(&mut self, body: Vec<u8>, content_type: &str) -> &mut Self {
        let content_type = if content_type.len() > 0 { content_type } else { "text/plain" };
        self.headers.insert(String::from("Content-Length"), body.len().to_string());
        self.headers.insert(String::from("Content-Type"), content_type.to_string());
        self.body = body;
        self
    }

    /// Sets the body of the response to a JSON value.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use krustie::{ response::{ HttpResponse, StatusCode }, request::HttpRequest, json::json };
    /// 
    /// fn get(request: &HttpRequest, response: &mut HttpResponse) {
    ///    response.json_body(json!({"message": "Hello, World!"}));
    /// }
    pub fn json_body(&mut self, data: JsonValue) -> &mut Self {
        let json = serde_json::to_string(&data).unwrap();
        self.body(json.as_bytes().to_vec(), "application/json");
        self
    }

    /// Gets the body of the response as a byte vector reference
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{response::{HttpResponse, StatusCode}, request::HttpRequest};
    ///
    /// fn get(request: &HttpRequest, response: &mut HttpResponse) {
    ///  let body = response.get_body();
    /// }
    pub fn get_body(&mut self) -> &Vec<u8> {
        &mut self.body
    }

    /// Gets the body of the response as a **mutable** byte vector reference
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{response::{HttpResponse, StatusCode}, request::HttpRequest};
    ///
    /// fn get(request: &HttpRequest, response: &mut HttpResponse) {
    ///  let body = response.get_body();
    /// }
    pub fn get_body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }
}
