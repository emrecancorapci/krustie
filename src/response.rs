use std::collections::HashMap;

pub mod status_code;
pub mod body;

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
    pub fn status(&mut self, status_code: StatusCode) -> &mut Self {
        self.status_code = status_code;
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
    /// use krustie::{response::{HttpResponse, StatusCode}, request::HttpRequest};
    ///
    /// fn get(request: &HttpRequest, response: &mut HttpResponse) {
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
    /// use krustie::{response::{HttpResponse, StatusCode}, request::HttpRequest};
    ///
    /// fn get(request: &HttpRequest, response: HttpResponse) {
    ///   let headers = response.get_headers();
    ///
    ///   for (key, value) in headers.iter() {
    ///    println!("{}: {}", key, value);
    ///   }
    /// }
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Allows to set the debug mode for the response. If debug mode is set to true, all debug messages will be printed to the console.
    pub(crate) fn debug_msg(&mut self, msg: &str) -> &mut Self {
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

        if self.body.len() > 0 {
            response_bytes.extend_from_slice(&self.body);
        }


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
        Self {
            debug_mode: false,
            http_version: "HTTP/1.1".to_string(),
            status_code: StatusCode::NotFound,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    RequestTimeout = 408,
    LengthRequired = 411,
    UnsupportedMediaType = 415,
    IAmATeapot = 418,
    InternalServerError = 500,
    NotImplemented = 501,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
}
