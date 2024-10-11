//! This module contains the `Response` struct and its implementation.
//!
//! Because they are mutable, they don't need to be returned from the controllers.
//!
//! Response can be basicaly built by using the `get`, `headers` and `body` functions which can be
//! chained like this:
//!
//! ```rust
//! use krustie::{ Response, StatusCode, Request, response::ContentType, };
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
//!     .body(b"Hello, World!".to_vec(), ContentType::Text);
//! }
//! ```
//!
//! But there are other functions such as `set_header` and `set_body` can be useful especially when creating a middleware.
//!
use self::status_code::StatusCode;
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter, Result},
};

pub use self::content_type::ContentType;

pub mod body;
pub mod content_type;
pub mod status_code;
pub mod utilities;

/// Represents the HTTP response
///
/// In Krustie, reponse objects doesn't need to be returned and created by developer.
/// Instead they need to be modified using functions.
///
/// The basic functions to create a response are: `status`, `headers`, `body` and `body_json`.
///
/// - `status` *sets* the status code of the response. It takes [StatusCode] as an argument.
/// - `headers` *extends* the current headers of the response. It takes a `HashMap<string, string>` as an argument.
///   - If you want to add just *a single header* there is the `set_header` function which takes two strings, a key and a value, as an argument.
/// - `body`
///
pub struct Response {
    debug_mode: bool,
    http_version: String,
    status_code: StatusCode,
    headers: HashMap<String, String>,
    locals: HashMap<String, String>,
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
    /// Add `Server: Rust` and `Connection: close` headers to the response.
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
    pub fn headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        self.headers.extend(headers);
        self
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

impl From<Response> for Vec<u8> {
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
    fn from(response: Response) -> Vec<u8> {
        let mut headers = response.headers;
        let mut headers_string = String::new();

        if !response.body.is_empty() {
            headers.insert(
                "Content-Length".to_string(),
                response.body.len().to_string(),
            );

            if !headers.contains_key("Content-Type") {
                eprintln!("Content-Type not found even though body is present");
                headers.insert("Content-Type".to_string(), "text/plain".to_string());
            }
        }

        if !headers.is_empty() {
            let mut headers_vec: Vec<String> = Vec::new();

            headers.iter().for_each(|(key, value)| {
                headers_vec.push(format!("{key}: {value}"));
            });

            headers_string = headers_vec.join("\r\n");
        }

        let mut response_bytes: Vec<u8> = format!(
            "{http_version} {status_code} {status_msg}\r\n{headers_string}\r\n",
            http_version = response.http_version,
            status_code = response.status_code,
            status_msg = response.status_code.get_message()
        )
        .into_bytes();

        if !response.body.is_empty() {
            response_bytes.extend_from_slice(b"\r\n");
            response_bytes.extend_from_slice(&response.body);
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
    /// let response = Response::default();
    /// ```
    fn default() -> Self {
        Self {
            debug_mode: false,
            http_version: "HTTP/1.1".to_string(),
            status_code: StatusCode::NotFound,
            headers: HashMap::new(),
            body: Vec::new(),
            locals: HashMap::new(),
        }
    }
}

impl Debug for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{http_version} {status_code} {status_msg}\r\n{headers}\r\n",
            http_version = self.http_version,
            status_code = self.status_code,
            status_msg = self.status_code.get_message(),
            headers = self
                .headers
                .iter()
                .fold(String::new(), |acc, (key, value)| format!(
                    "{acc}{key}: {value}\r\n"
                ))
        )
    }
}
