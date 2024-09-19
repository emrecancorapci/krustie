//! # Request
//! 
//! This module contains the `Request` struct and its implementation.
//!
//! It has public methods to get the headers, add and get local variables
//!
//! All request parameters are non-mutable except for the *local* variables. Local variables can be
//! used to store data that can be defined in a *middleware* and accessed from the other
//! *middlewares* or *controllers*.

use self::{http_method::HttpMethod, request_line::RequestLine};
use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter, Result as fResult},
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

pub use body::RequestBody;

pub mod body;
pub mod http_method;
pub(crate) mod parser;
mod request_line;

/// Represents the HTTP request
/// 
/// The `Request` struct contains the request line, headers, queries, parameters, body and peer address of the HTTP request.
#[derive(Clone)]
pub struct Request {
    request: RequestLine,
    headers: HashMap<String, String>,
    queries: HashMap<String, String>,
    params: HashMap<String, String>,
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

    /// Returns the value of the requested header
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
    /// The peer address is the ip address of the client that made the request
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

    /// Returns the queries of the HTTP request as a HashMap
    /// 
    /// | The path of the HTTP request | Value |
    /// | -- | -- |
    /// | `/hello` | `[]` |
    /// | `/hello?planet=earth` | `[{ "planet": "earth" }]` |
    /// | `/hello?planet=earth&moon=luna` | `[{ "planet": "earth" }, { "moon": "luna"}]` |
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use krustie::{ Request, Response };
    /// 
    /// fn get(request: &Request, response: &mut Response) {
    ///   let queries = request
    ///     .get_queries()
    ///     .iter()
    ///     .map(|(k, v)| format!("{}: {}", k, v))
    ///     .collect::<Vec<String>>();
    /// }
    /// ```
    pub fn get_queries(&self) -> &HashMap<String, String> {
        &self.queries
    }

    /// Returns the query parameter of the HTTP request
    /// 
    /// | The path of the HTTP request | get_query_param(key: &str) | Returns |
    /// | -- | -- | -- |
    /// | `/hello?planet=earth` | `get_query_param("planet")` | Some("earth") |
    /// | `/hello?planet=earth` | `get_query_param("moon")` | None |
    /// | `/hello?planet=earth&moon=luna` | `get_query_param("moon")` | Some("luna") |
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use krustie::{ Request, Response };
    /// 
    /// fn get(request: &Request, response: &mut Response) {
    ///   let id = request.get_query_param("id");
    /// }
    /// ```
    pub fn get_query_param(&self, key: &str) -> Option<&String> {
        self.queries.get(key)
    }

    /// Returns the path of the HTTP request as a Vector
    /// 
    /// | The path of the HTTP request | `get_path_array()` |
    /// | -- | -- |
    /// | `/` | `vec![]` |
    /// | `/hello` | `vec!["hello"]` |
    /// | `/hello/world` | `vec!["hello", "world"]` |
    /// | `/hello/world?city=istanbul` | `vec!["hello", "world?city=istanbul"]` |
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use krustie::{ Request, Response };
    /// 
    /// fn get(request: &Request, response: &mut Response) {
    ///   let path: Vec<String> = request.get_path_array();
    /// }
    /// ```
    pub fn get_path_array(&self) -> &Vec<String> {
        self.request.get_path_array()
    }

    /// Returns the path of the HTTP request as a String
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use krustie::{ Request, Response };
    /// 
    /// fn get(request: &Request, response: &mut Response) {
    ///   let path: &String = request.get_path();
    /// }
    /// ```
    pub fn get_path(&self) -> &String {
        self.request.get_path()
    }

    /// Returns the requested parameter of the HTTP request
    /// 
    /// | Route | Path | get_param(key: &str) | Returns |
    /// | -- | -- | -- | -- |
    /// | `/hello/:name` | `/hello/marvin` | `get_param("name")` | `Some("marvin")` |
    /// | `/hello/:name` | `/hello/marvin` | `get_param("planet")` | `None` |
    /// | `/hello/:name/:planet` | `/hello/marvin/earth` | `get_param("planet")` | `Some("earth")` |
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use krustie::{ Request, Response };
    /// 
    /// fn get(request: &Request, response: &mut Response) {
    ///   let sort: Option<&String> = request.get_param("sort");
    /// }
    /// ```
    pub fn get_param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }

    /// Returns the method of the HTTP request
    pub(crate) fn get_method(&self) -> &HttpMethod {
        self.request.get_method()
    }

    pub(crate) fn add_param(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }
}

impl Default for Request {
    fn default() -> Self {
        Self {
            request: RequestLine::new("GET", "/", "HTTP/1.1")
                .expect("Failed to create default RequestLine"),
            queries: HashMap::new(),
            headers: HashMap::new(),
            params: HashMap::new(),
            body: RequestBody::None,
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
        }
    }
}

impl Debug for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("  {k}: {v}"))
            .collect::<Vec<String>>()
            .join("\r\n");
        let params = self
            .params
            .iter()
            .map(|(k, v)| format!("  {k}: {v}"))
            .collect::<Vec<String>>()
            .join("\r\n");
        let queries = self
            .queries
            .iter()
            .map(|(k, v)| format!("  {k}: {v}"))
            .collect::<Vec<String>>()
            .join("\r\n");
        let body = match &self.body {
            RequestBody::Text(body) => format!("{:?}", body),
            RequestBody::Json(json) => format!("{:?}", json),
            RequestBody::None => "None".to_string(),
        };

        write!(
            f,
            "From:\r\n  {}\r\nRequest Line:\r\n  {}\r\nHeaders:\r\n{}\r\nParams:\r\n{}\r\nQueries:\r\n{}\r\nBody:\r\n{}",
            self.peer_addr,
            self.request,
            headers,
            params,
            queries,
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
