//! Body module for the Response struct. Contains functions for setting the body of the
//! response.

use super::Response;
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
    /// use krustie::{ Response, StatusCode, Request, json::json };
    ///
    /// fn get(request: &Request, response: &mut Response) {
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
    /// use krustie::{ Response, StatusCode, Request, json::json };
    ///
    /// fn get(request: &Request, response: &mut Response) {
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
    /// use krustie::{ Response, StatusCode, Request, json::json };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///  let body = response.get_body();
    /// }
    pub fn get_body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }

    /// Updates the body of the response.
    ///
    /// Function sets `Content-Length` automatically but needs `Content-Type` to be set manually.
    ///
    /// # Errors
    ///
    /// Returns an error if the request has no body already.
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request, json::json };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///   response.body(b"Hello, World!".to_vec(), "text/plain");
    ///
    ///   response.update_body(b"Goodbye, Mars!".to_vec());
    /// }
    pub fn update_body(&mut self, body: Vec<u8>) -> Result<(), String> {
        if self.body.len() == 0 {
            return Err("Request has no body.".to_string());
        }
        self.headers.insert(String::from("Content-Length"), body.len().to_string());
        self.body = body;
        return Ok(());
    }
}
