use std::{ collections::HashMap, fmt::Debug };

use self::request_line::RequestLine;

pub mod http_method;

mod request_line;

pub struct HttpRequest<'a> {
    pub request: RequestLine<'a>,
    pub headers: HashMap<String, String>,
    pub body: &'a str,
}

impl<'a> HttpRequest<'a> {
    /// Creates a new HttpRequest
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::request::{HttpRequest, HttpMethod};
    ///
    /// let header = vec!["GET / HTTP/1.1".to_string()];
    /// let request = HttpRequest::new(&header, "");
    ///
    /// assert_eq!(request.request.method, HttpMethod::GET);
    /// ```
    pub fn new(http_request: &'a Vec<String>, body: &'a str) -> HttpRequest<'a> {
        let request = (
            {
                match http_request.first() {
                    Some(request_line) => { RequestLine::new_from_str(request_line) }
                    None => { todo!("Implement none handling") }
                }
            }
        ).expect("RequestLine not found");

        let headers = http_request
            .iter()
            .skip(1)
            .filter_map(HttpRequest::header_parser())
            .collect();

        HttpRequest {
            request,
            headers,
            body,
        }
    }

    fn header_parser() -> impl Fn(&String) -> Option<(String, String)> {
        |line: &String| {
            let header_line: Vec<&str> = line.split(':').collect();

            if header_line.len() == 2 {
                let key = header_line[0].trim().to_lowercase().to_string();
                let value = header_line[1].trim().to_string();

                Some((key, value))
            } else {
                None
            }
        }
    }
}

impl Default for HttpRequest<'_> {
    fn default() -> Self {
        HttpRequest {
            request: RequestLine::new("GET", "/", "HTTP/1.1"),
            headers: HashMap::new(),
            body: ""
        }
    }
}

impl Debug for HttpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers = self.headers
            .iter()
            .fold(String::new(), |acc, (k, v)| format!("{acc}{k}: {v}\r\n"));

        write!(
            f,
            "HttpRequest Line: {}\r\n Headers: {}\r\n Body: {}",
            self.request.to_string(),
            headers,
            self.body
        )
    }
}


#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    // CONNTECT,
    // HEAD,
    // OPTIONS,
    // TRACE,
}

impl HttpMethod {
    /// Create a new HttpMethod from a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::request::HttpMethod;
    ///
    /// let method = HttpMethod::new("GET").expect("Method not found");
    ///
    /// assert_eq!(method, HttpMethod::GET);
    /// ```
    pub fn new(method: &str) -> Result<HttpMethod, &str> {
        let binding = method.to_lowercase();

        match binding.as_str() {
            "get" => {
                return Ok(HttpMethod::GET);
            }
            "post" => {
                return Ok(HttpMethod::POST);
            }
            "put" => {
                return Ok(HttpMethod::PUT);
            }
            "patch" => {
                return Ok(HttpMethod::PATCH);
            }
            "delete" => {
                return Ok(HttpMethod::DELETE);
            }
            &_ => { Err("Method not found.") }
        }
    }
}
