//! RequestBuilder module
//!
//! This module contains the `RequestBuilder` struct and its implementation.

use crate::HttpMethod;

use super::{Request, RequestBody};

/// RequestBuilder
///
/// A builder for the `Request` struct
///
/// This builder is used to create a `Request` struct to be used in testing.
///
/// # Example
///
/// ```
/// use krustie::{HttpMethod, Request, request::RequestBody};
///
/// let request = Request::builder()
///   .method(HttpMethod::GET)
///   .path("/echo/hello")
///   .header("Content-Type", "application/json")
///   .body(RequestBody::Text("Hello, World!".to_string()))
///   .build();
///
/// assert_eq!(request.get_method(), &HttpMethod::GET);
/// assert_eq!(request.get_path(), "/echo/hello");
/// assert_eq!(request.get_header("Content-Type"), Some("application/json"));
/// assert_eq!(request.get_body(), &RequestBody::Text("Hello, World!".to_string()));
/// ```
#[derive(Debug)]
pub struct RequestBuilder {
    request: Request,
}

impl RequestBuilder {
    fn new() -> Self {
        Self {
            request: Request::default(),
        }
    }

    /// Sets the request line of the HTTP request
    ///
    /// # Example
    ///
    /// ```
    /// use krustie::{HttpMethod, Request};
    ///
    /// let request = Request::builder()
    ///   .request_line(HttpMethod::GET, "/echo/hello", "HTTP/1.1")
    ///   .build();
    ///
    /// assert_eq!(request.get_method(), &HttpMethod::GET);
    /// assert_eq!(request.get_path(), "/echo/hello");
    /// assert_eq!(request.get_version(), "HTTP/1.1");
    /// ```
    pub fn request_line(&mut self, method: HttpMethod, path: &str, version: &str) -> &mut Self {
        self.request.set_method(method);
        self.request.set_uri(path);
        self.request.set_version(version);
        self
    }

    /// Sets the method of the HTTP request
    ///
    /// # Example
    ///
    /// ```
    /// use krustie::{HttpMethod, Request};
    ///
    /// let request = Request::builder()
    ///   .method(HttpMethod::GET)
    ///   .build();
    ///
    /// assert_eq!(request.get_method(), &HttpMethod::GET);
    /// ```
    pub fn method(&mut self, method: HttpMethod) -> &mut Self {
        self.request.set_method(method);
        self
    }

    /// Sets the path of the HTTP request
    ///
    /// # Example
    ///
    /// ```
    /// use krustie::{HttpMethod, Request};
    ///
    /// let request = Request::builder()
    ///   .path("/echo/hello")
    ///   .build();
    ///
    /// assert_eq!(request.get_path(), "/echo/hello");
    /// ```
    pub fn path(&mut self, path: &str) -> &mut Self {
        self.request.set_uri(path);
        self
    }

    /// Sets the version of the HTTP request
    ///
    /// # Example
    ///
    /// ```
    /// use krustie::{HttpMethod, Request};
    ///
    /// let request = Request::builder()
    ///   .version("HTTP/1.1")
    ///   .build();
    ///
    /// assert_eq!(request.get_version(), "HTTP/1.1");
    /// ```
    pub fn version(&mut self, version: &str) -> &mut Self {
        self.request.set_version(version);
        self
    }

    /// Sets the header of the HTTP request
    ///
    /// # Example
    ///
    /// ```
    /// use krustie::{HttpMethod, Request};
    ///
    /// let request = Request::builder()
    ///   .header("Content-Type", "application/json")
    ///   .build();
    ///
    /// assert_eq!(request.get_header("Content-Type"), Some("application/json"))
    /// ```
    pub fn header(&mut self, key: &str, value: &str) -> &mut Self {
        self.request.set_header(key, value);
        self
    }

    /// Sets the headers of the HTTP request
    ///
    /// # Example
    ///
    /// ```
    /// use krustie::{HttpMethod, Request};
    ///
    /// let request = Request::builder()
    ///   .headers(vec![("Content-Type", "application/json"), ("Server", "Krustie")])
    ///   .build();
    ///
    /// assert_eq!(request.get_header("Content-Type"), Some("application/json"));
    /// assert_eq!(request.get_header("Server"), Some("Krustie"));
    /// ```
    pub fn headers(&mut self, headers: Vec<(&str, &str)>) -> &mut Self {
        for (key, value) in headers {
            self.request.set_header(key, value);
        }
        self
    }

    /// Sets the body of the HTTP request
    ///
    /// # Example
    ///
    /// ```
    /// use krustie::{HttpMethod, Request, request::RequestBody};
    ///
    /// let request = Request::builder()
    ///   .body(RequestBody::Text("Hello, World!".to_string()))
    ///   .build();
    ///
    /// assert_eq!(request.get_body(), &RequestBody::Text("Hello, World!".to_string()));
    /// ```
    pub fn body(&mut self, body: RequestBody) -> &mut Self {
        self.request.set_body(body);
        self
    }

    /// Builds the `Request` struct
    ///
    /// # Example
    ///
    /// ```
    /// use krustie::{HttpMethod, Request};
    ///
    /// let request = Request::builder()
    ///   .method(HttpMethod::GET)
    ///   .path("/echo/hello")
    ///   .build();
    ///
    /// assert_eq!(request.get_method(), &HttpMethod::GET);
    /// assert_eq!(request.get_path(), "/echo/hello");
    /// ```
    pub fn build(&self) -> Request {
        self.request.clone()
    }
}

impl Request {
    /// Returns a new RequestBuilder instance
    /// 
    /// # Example
    /// 
    /// ```
    /// use krustie::{Request, HttpMethod};
    /// 
    /// let request = Request::builder()
    ///   .method(HttpMethod::GET)
    ///   .build();
    /// 
    /// assert_eq!(request.get_method(), &HttpMethod::GET);
    /// ```
    pub fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }

    fn set_method(&mut self, method: HttpMethod) {
        self.request.set_method(method);
    }

    fn set_uri(&mut self, path: &str) {
        self.request.set_uri(path);
        self.queries = Request::parse_queries(&self.request.get_path_array());
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
