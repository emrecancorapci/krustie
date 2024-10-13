//! RequestBuilder module
//!
//! This module contains the `RequestBuilder` struct and its implementation.

use crate::HttpMethod;

use super::{Request, RequestBody};

#[derive(Debug)]
pub struct RequestBuilder {
    request: Request,
}

/// A builder for the `Request` struct
///
/// This builder is used to create a `Request` struct to be used in testing.
impl RequestBuilder {
    fn new() -> Self {
        Self {
            request: Request::default(),
        }
    }

    pub fn request_line(&mut self, method: HttpMethod, path: &str, version: &str) -> &mut Self {
        self.request.set_method(method);
        self.request.set_uri(path);
        self.request.set_version(version);
        self
    }

    pub fn method(&mut self, method: HttpMethod) -> &mut Self {
        self.request.set_method(method);
        self
    }

    pub fn path(&mut self, path: &str) -> &mut Self {
        self.request.set_uri(path);
        self
    }

    pub fn version(&mut self, version: &str) -> &mut Self {
        self.request.set_version(version);
        self
    }

    pub fn header(&mut self, key: &str, value: &str) -> &mut Self {
        self.request.set_header(key, value);
        self
    }

    pub fn headers(&mut self, headers: Vec<(&str, &str)>) -> &mut Self {
        for (key, value) in headers {
            self.request.set_header(key, value);
        }
        self
    }

    pub fn body(&mut self, body: RequestBody) -> &mut Self {
        self.request.set_body(body);
        self
    }

    pub fn build(&self) -> Request {
        self.request.clone()
    }
}

impl Request {
    // TODO: Add doctest
    /// Returns a new RequestBuilder instance
    pub fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }

    fn set_method(&mut self, method: HttpMethod) {
        self.request.set_method(method);
    }

    fn set_uri(&mut self, path: &str) {
        self.request.set_uri(path);
    }

    fn set_version(&mut self, version: &str) {
        self.request.set_version(version);
    }

    fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    fn set_body(&mut self, body: RequestBody) {
        self.body = body;
    }
}
