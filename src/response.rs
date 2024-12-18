use self::status_code::StatusCode;
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter, Result},
};

pub use self::content_type::ContentType;

pub mod body;
pub mod content_type;
pub mod status_code;
pub mod testing;
pub mod utilities;

#[doc = include_str!("../docs/core/response.md")]
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

    fn export_request_header(&self) -> String {
        let mut headers = self.headers.clone();
        let mut headers_string = String::new();

        if !self.body.is_empty() {
            headers.insert("Content-Length".to_string(), self.body.len().to_string());

            if !headers.contains_key("Content-Type") {
                eprintln!("Content-Type not found even though body is present");

                headers.insert("Content-Type".to_string(), "text/plain".to_string());
            }
        }

        if !headers.is_empty() {
            let mut keys = headers.keys().collect::<Vec<&String>>();
            keys.sort();

            headers_string = keys.iter().fold(String::new(), |acc, key| {
                format!("{acc}{key}: {value}\r\n", value = headers[*key])
            });
        }

        return format!(
            "{http_version} {status_code} {status_msg}\r\n{headers_string}\r\n",
            http_version = self.http_version,
            status_code = self.status_code,
            status_msg = self.status_code.get_message()
        );
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut response_string = self.export_request_header();

        if !self.body.is_empty() {
            response_string.push_str(&String::from_utf8_lossy(&self.body));
        }

        response_string
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
        let mut response_bytes = response.export_request_header().into_bytes();

        if !response.body.is_empty() {
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
