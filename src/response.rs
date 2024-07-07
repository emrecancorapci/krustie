//! This module contains the `Response` struct and its implementation.
//!
//! Because they are mutable, they don't need to be returned from the controllers.
//!
//! Response can be basicaly built by using the `get`, `headers` and `body` functions which can be
//! chained like this:
//!
//! ```rust
//! use krustie::{ Response, StatusCode, Request };
//! use std::collections::HashMap;
//!
//! fn get(request: &Request, response: &mut Response) {
//!   let mut headers = HashMap::new();
//!
//!   headers.insert("Server".to_string(), "Krustie".to_string());
//!
//!   response
//!     .status(StatusCode::Ok)
//!     .headers(headers)
//!     .body(b"Hello, World!".to_vec(), "text/plain");
//! }
//! ```
//!
//! But there are other functions such as `insert_header` and `update_body` can be useful especially when creating a middleware.
//!
use std::{ collections::HashMap, fmt::{ Debug, Formatter, Result } };

use self::status_code::StatusCode;

pub mod status_code;
pub mod body;

/// Represents the HTTP response
///
///
pub struct Response {
    debug_mode: bool,
    http_version: String,
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    /// Sets the status of the response
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///    response.status(StatusCode::Ok);
    /// }
    /// ```
    pub fn status(&mut self, status_code: StatusCode) -> &mut Self {
        self.status_code = status_code;
        self
    }

    /// Adds headers to the response
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request };
    /// use std::collections::HashMap;
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///     let mut headers = HashMap::new();
    ///
    ///     headers.insert("Server".to_string(), "Krustie".to_string());
    ///     headers.insert("Connection".to_string(), "close".to_string());
    ///
    ///     response.status(StatusCode::Ok).headers(headers);
    /// }
    /// ```
    ///
    /// Code above will add `Server: Rust` and `Connection: close` headers to the response.
    pub fn headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        self.headers.extend(headers);
        self
    }

    /// Adds a single header to the response
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///    response.insert_header("Server", "Krustie");
    /// }
    /// ```
    pub fn insert_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

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
    ///    println!("{}: {}", key, value);
    ///   }
    /// }
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Allows to set the debug mode for the response.
    ///
    /// If `debug_mode` is set to `true`, all debug messages will be printed to the console.
    pub(crate) fn debug_msg(&mut self, msg: &str) -> &mut Self {
        if self.debug_mode {
            println!("{}", msg);
        }
        self
    }
}

impl Into<Vec<u8>> for Response {
    /// Returns the response as a byte vector.
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request };
    ///
    /// fn get(request: &Request, response: Response) {
    ///   let response_bytes: Vec<u8> = response.into();
    /// }
    /// ```
    fn into(self) -> Vec<u8> {
        let mut headers_string = String::new();

        if self.headers.len() > 0 {
            self.headers.iter().for_each(|(key, value)| {
                headers_string.push_str(&format!("{key}: {value}\r\n"));
            });
        }

        if self.body.len() > 0 {
            if !headers_string.contains("Content-Length") {
                eprintln!("Content-Length not found even though body is present");
                headers_string.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
            }
            if !headers_string.contains("Content-Type") {
                eprintln!("Content-Type not found even though body is present");
                headers_string.push_str("Content-Type: text/plain\r\n");
            }
        }

        let mut response_bytes: Vec<u8> = format!(
            "{http_version} {status_code} {status_msg}\r\n{headers_string}\r\n",
            http_version = self.http_version,
            status_code = self.status_code.to_string(),
            status_msg = self.status_code.get_message()
        ).into_bytes();

        if self.body.len() > 0 {
            response_bytes.extend_from_slice(&self.body);
        }

        response_bytes
    }
}

impl Default for Response {
    /// Returns a default instance of Response
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::response::Response;
    ///
    /// fn main() {
    ///     let response = Response::default();
    /// }
    /// ```
    fn default() -> Self {
        Self {
            debug_mode: false,
            http_version: "HTTP/1.1".to_string(),
            status_code: StatusCode::NotFound,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}

impl Debug for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{http_version} {status_code} {status_msg}\r\n{headers}\r\n",
            http_version = self.http_version,
            status_code = self.status_code.to_string(),
            status_msg = self.status_code.get_message(),
            headers = self.headers
                .iter()
                .map(|(key, value)| format!("{key}: {value}\r\n"))
                .collect::<String>()
        )
    }
}
