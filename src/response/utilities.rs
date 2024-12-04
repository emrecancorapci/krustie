//! # Response utilities
//!
//! This module contains utility functions for the response object.

use std::collections::HashMap;

use crate::{Response, StatusCode};

impl Response {
    /// Gets the headers of the response
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request };
    ///
    /// fn get(request: &Request, response: Response) {
    ///   let headers = response.get_headers();
    ///
    ///   for (key, value) in headers.iter() {
    ///     println!("{}: {}", key, value);
    ///   }
    /// }
    /// ```
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Gets requested header from the response
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request };
    ///
    /// fn get(request: &Request, response: Response) {
    ///   let server_header = response.get_header("Server");
    ///
    ///   match server_header {
    ///     Some(header) => println!("Server header: {}", header),
    ///     None => println!("Server header not found")
    ///   }
    /// }
    /// ```
    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    /// Adds a single header to the response
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///   response.set_header("Server", "Krustie");
    /// }
    /// ```
    pub fn set_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
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
    ///   let body = response.get_body();
    /// }
    /// ```
    pub fn get_body(&self) -> &Vec<u8> {
        &self.body
    }

    /// Gets the body of the response as a **mutable** byte vector reference
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request, json::json };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///   let body = response.get_body_mut();
    /// }
    /// ```
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
    /// use krustie::{ Response, StatusCode, Request, response::ContentType, json::json };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///   response.body(b"Hello, World!".to_vec(), ContentType::Text);
    ///
    ///   response.set_body(b"Goodbye, Mars!".to_vec());
    /// }
    /// ```
    pub fn set_body(&mut self, body: Vec<u8>) -> Result<(), String> {
        if self.body.is_empty() {
            return Err("Request has no body.".to_string());
        }

        self.body = body;
        return Ok(());
    }

    /// Returns the value of the local variable
    ///
    /// `Local` variables can be used to store data that can be defined in a *middleware* and accessed in the *controller*
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Request, Response };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///    let user_id = response.get_local("user_id");
    /// }
    /// ```
    pub fn get_local(&self, key: &str) -> Option<&String> {
        self.locals.get(key)
    }

    /// Adds a local variable to the http request
    ///
    /// `Local` variables can be used to store data that can be defined in a *middleware* and accessed in the *controller*
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Request, Response };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///   response.set_local("user_id", "123");
    /// }
    /// ```
    pub fn set_local(&mut self, key: &str, value: &str) {
        self.locals.insert(key.to_string(), value.to_string());
    }

    /// Returns the status code of the response
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request };
    ///
    /// fn get(request: &Request, response: Response) {
    ///   let status_code = response.get_status();
    /// }
    /// ```
    pub fn get_status(&self) -> StatusCode {
        self.status_code
    }

    /// Returns true if status code is 4xx or 5xx.
    pub fn is_error(&self) -> bool {
        match self.status_code {
            StatusCode::BadRequest => true,
            StatusCode::Unauthorized => true,
            StatusCode::Forbidden => true,
            StatusCode::NotFound => true,
            StatusCode::MethodNotAllowed => true,
            StatusCode::RequestTimeout => true,
            StatusCode::LengthRequired => true,
            StatusCode::UnsupportedMediaType => true,
            StatusCode::IAmATeapot => true,
            StatusCode::TooManyRequests => true,
            StatusCode::InternalServerError => true,
            StatusCode::NotImplemented => true,
            StatusCode::ServiceUnavailable => true,
            StatusCode::GatewayTimeout => true,
            StatusCode::HttpVersionNotSupported => true,
            _ => false,
        }
    }
}
