use std::collections::HashMap;

pub mod status_code;

pub struct HttpResponse {
    debug_mode: bool,
    http_version: String,
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl HttpResponse {
    /// Sets the status of the response
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{response::{HttpResponse, StatusCode}, request::HttpRequest};
    ///
    /// fn get(request: &HttpRequest, response: &mut HttpResponse) {
    ///    response.status(StatusCode::Ok);
    /// }
    /// ```
    pub fn status(&mut self, status_code: StatusCode) -> &mut HttpResponse {
        self.status_code = status_code;
        self
    }

    /// Sets the body of the response. Function sets `Content-Length` automatically but needs `Content-Type` to be set manually.
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{response::{HttpResponse, StatusCode}, request::HttpRequest};
    ///
    /// fn get(request: &HttpRequest, response: &mut HttpResponse) {
    ///     response.body(b"Hello, World!".to_vec(), "text/plain");
    /// }
    /// ```
    ///
    /// ```rust
    /// use krustie::{response::{HttpResponse, StatusCode}, request::HttpRequest};
    ///
    /// fn get(request: &HttpRequest, response: &mut HttpResponse) {
    ///    response.status(StatusCode::Ok).body(b"<html><body><h1>Hello, World!</h1></body></html>".to_vec(), "text/html");
    /// }
    /// ```
    pub fn body(&mut self, body: Vec<u8>, content_type: &str) -> &mut HttpResponse {
        self.body = body.clone();
        self.headers.insert(String::from("Content-Length"), body.len().to_string());
        self.headers.insert(String::from("Content-Type"), content_type.to_string());
        self
    }

    /// Adds headers to the response
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use krustie::{response::{HttpResponse, StatusCode}, request::HttpRequest};
    ///
    /// fn get(request: &HttpRequest, response: &mut HttpResponse) {
    ///     let mut headers = HashMap::new();
    ///
    ///     headers.insert("Server".to_string(), "Rust".to_string());
    ///     headers.insert("Connection".to_string(), "close".to_string());
    ///
    ///     response.status(StatusCode::Ok).headers(headers);
    /// }
    /// ```
    ///
    /// Code above will add `Server: Rust` and `Connection: close` headers to the response.
    pub fn headers(&mut self, headers: HashMap<String, String>) -> &mut HttpResponse {
        self.headers.extend(headers);
        self
    }

    pub fn debug_msg(&mut self, msg: &str) -> &mut HttpResponse {
        if self.debug_mode {
            println!("{}", msg);
        }
        self
    }
}

impl Into<Vec<u8>> for HttpResponse {
    /// Returns the response as a byte vector.
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{response::{HttpResponse, StatusCode}, request::HttpRequest};
    ///
    /// fn get(request: &HttpRequest, response: HttpResponse) {
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
                println!("Content-Length not found even though body is present");
                headers_string.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
            }
            if !headers_string.contains("Content-Type") {
                println!("Content-Type not found even though body is present");
                headers_string.push_str("Content-Type: text/plain\r\n");
            }
        }

        let mut response_bytes: Vec<u8> = format!(
            "{http_version} {status_code} {status_msg}\r\n{headers_string}\r\n",
            http_version = self.http_version,
            status_code = self.status_code.to_string(),
            status_msg = self.status_code.get_message()
        ).into_bytes();

        response_bytes.extend_from_slice(&self.body);

        response_bytes
    }
}

impl Default for HttpResponse {
    /// Returns a default instance of HttpResponse
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::response::HttpResponse;
    ///
    /// fn main() {
    ///     let response = HttpResponse::default();
    /// }
    /// ```
    fn default() -> Self {
        HttpResponse {
            debug_mode: false,
            http_version: "HTTP/1.1".to_string(),
            status_code: StatusCode::NotFound,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum StatusCode {
    Ok,
    Created,
    Accepted,
    NoContent,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    RequestTimeout,
    LengthRequired,
    UnsupportedMediaType,
    IAmATeapot,
    InternalServerError,
    NotImplemented,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
}
