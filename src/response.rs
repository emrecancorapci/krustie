use std::collections::HashMap;

pub struct HttpResponse {
    debug_mode: bool,
    http_version: String,
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl HttpResponse {
    pub fn new() -> HttpResponse {
        HttpResponse {
            debug_mode: false,
            http_version: "HTTP/1.1".to_string(),
            status_code: StatusCode::Ok,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
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
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut headers = String::new();

        if self.headers.len() > 0 {
            for (key, value) in &self.headers {
                headers.push_str(&format!("{key}: {value}\r\n"));
            }
        }

        if self.body.len() > 0 {
            if !headers.contains("Content-Length") {
                println!("Content-Length not found even though body is present");
                headers.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
            }
            if !headers.contains("Content-Type") {
                println!("Content-Type not found even though body is present");
                headers.push_str("Content-Type: text/plain\r\n");
            }
        }

        let mut response_bytes = format!(
            "{http_version} {status_code} {status_msg}\r\n{headers}\r\n",
            http_version = self.http_version,
            status_code = self.status_code.code(),
            status_msg = self.status_code.msg()
        ).into_bytes();

        response_bytes.extend_from_slice(&self.body);

        response_bytes
    }
    pub fn debug_on(&mut self) -> &mut HttpResponse {
        self.debug_mode = true;
        self
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum StatusCode {
    Ok,
    Created,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    IAmATeapot,
    InternalServerError,
}

impl StatusCode {
    pub fn code(&self) -> &str {
        match self {
            StatusCode::Ok => "200",
            StatusCode::Created => "201",
            StatusCode::BadRequest => "400",
            StatusCode::NotFound => "404",
            StatusCode::MethodNotAllowed => "405",
            StatusCode::IAmATeapot => "418",
            StatusCode::InternalServerError => "500",
        }
    }

    pub fn msg(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::IAmATeapot => "I'm a teapot",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}
