//! This module contains the `Request` struct and its implementation.
//!
//! It has public methods to get the headers, add and get local variables
//!
//! All request parameters are non-mutable except for the *local* variables. Local variables can be
//! used to store data that can be defined in a *middleware* and accessed from the other
//! *middlewares* or *controllers*.

use std::{
    collections::HashMap,
    fmt::{ Debug, Display, Formatter, Result as fResult },
    net::{ IpAddr, Ipv4Addr, SocketAddr },
};
use self::{ http_method::HttpMethod, request_line::RequestLine };

pub use body::RequestBody;

pub mod body;
pub mod http_method;
pub(crate) mod parser;
mod request_line;

/// Represents the HTTP request
pub struct Request {
    request: RequestLine,
    headers: HashMap<String, String>,
    body: RequestBody,
    peer_addr: SocketAddr,
}

impl Request {
    /// Returns the reference of the Request headers as a HashMap
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ request::Request, response::Response};
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///   let headers = request.get_headers();
    ///   let content_type = headers.get("content-type");
    /// }
    /// ```
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Returns the value of the header key
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ request::Request, response::Response};
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///   let content_type = request.get_header("content-type");
    /// }
    /// ```
    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    /// Returns the body of the HTTP request
    ///
    /// The body can be of type `Text`, `Json`, `Form` or `None`
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Request, Response, request::RequestBody };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///   match request.get_body() {
    ///     RequestBody::Text(body) => {
    ///       // Do something with the body
    ///     },
    ///     RequestBody::Json(json) => {
    ///      // Do something with the json
    ///     },
    ///     _ => {
    ///      // Do something else
    ///     }
    ///   }
    /// }
    /// ```
    pub fn get_body(&self) -> &RequestBody {
        &self.body
    }

    /// Returns the peer address of the HTTP request
    ///
    /// The peer address is the address of the client that made the request
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Request, Response };
    /// use std::net::SocketAddr;
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///   let peer_addr: &SocketAddr = request.get_peer_addr();
    /// }
    /// ```
    pub fn get_peer_addr(&self) -> &SocketAddr {
        &self.peer_addr
    }

    /// Returns the method of the HTTP request
    pub(crate) fn get_method(&self) -> &HttpMethod {
        self.request.get_method()
    }

    /// Returns the path of the HTTP request
    pub(crate) fn get_path_array(&self) -> &Vec<String> {
        self.request.get_path_array()
    }
}

impl Default for Request {
    fn default() -> Self {
        Self {
            request: RequestLine::new("GET", "/", "HTTP/1.1").expect(
                "Failed to create default RequestLine"
            ),
            headers: HashMap::new(),
            body: RequestBody::None,
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
        }
    }
}

impl Debug for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        let headers = self.headers
            .iter()
            .fold(String::new(), |acc, (k, v)| format!("{acc}{k}: {v}\r\n"));

        let body = match &self.body {
            RequestBody::Text(body) => format!("{:?}", body),
            RequestBody::Json(json) => format!("{:?}", json),
            RequestBody::None => "None".to_string(),
        };

        write!(
            f,
            "From: {}\r\n Request Line: {}\r\n Headers: {}\r\n Body: {}",
            self.peer_addr,
            self.request,
            headers,
            body
        )
    }
}

#[derive(Debug)]
/// Represents an error that occurs when parsing an HTTP request
pub struct ParseHttpRequestError;

impl Display for ParseHttpRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        write!(f, "Failed to parse HTTP request")
    }
}
