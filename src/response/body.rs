//! Body module for the Response struct. Contains functions for setting the body of the
//! response.

use super::{content_type::ContentType, Response};
use serde_json::Value as JsonValue;

impl Response {
    /// Sets the body of the response. Function sets `Content-Length` automatically but needs `Content-Type` to be set manually.
    ///
    /// If `Content-Type` is not set, it defaults to `text/plain`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request, json::json };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///     response.body(b"Hello, World!".to_vec(), "text/plain");
    /// }
    /// ```
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request, json::json };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///    response.status(StatusCode::Ok).body(b"<html><body><h1>Hello, World!</h1></body></html>".to_vec(), "text/html");
    /// }
    /// ```
    pub fn body(&mut self, body: Vec<u8>, content_type: ContentType) -> &mut Self {
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
    /// use krustie::{ Response, StatusCode, Request, json::json };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///    response.body_json(json!({"message": "Hello, World!"}));
    /// }
    pub fn body_json(&mut self, data: JsonValue) -> &mut Self {
        let json = serde_json::to_string(&data).unwrap();
        self.body(json.as_bytes().to_vec(), ContentType::Json);
        self
    }
}
