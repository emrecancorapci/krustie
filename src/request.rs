use std::{ collections::HashMap, fmt::{ Debug, Display, Formatter, Result as fResult } };
use self::request_line::RequestLine;

pub mod http_method;
mod request_line;
pub(crate) mod parser;

pub struct HttpRequest {
    request: RequestLine,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl HttpRequest {
    /// Creates a new HttpRequest
    fn new(http_request: &Vec<String>, body: Option<&str>) -> Result<Self, ParseHttpRequestError> {
        if http_request.is_empty() {
            return Err(ParseHttpRequestError);
        }

        let request = RequestLine::try_from(http_request[0].as_str());

        let headers = http_request
            .iter()
            .skip(1)
            .filter_map(HttpRequest::header_parser())
            .collect();

        let body = match body {
            Some(body) => Some(body.as_bytes().to_vec()),
            None => None,
        };

        match request {
            Ok(request) =>
                Ok(HttpRequest {
                    request,
                    headers,
                    body,
                }),
            Err(_) => Err(ParseHttpRequestError),
        }
    }

    /// Returns the reference of the HTTPRequest headers as a HashMap
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ request::HttpRequest, response::HttpResponse};
    ///
    /// fn get(request: &HttpRequest, response: &mut HttpResponse) {
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
    /// use krustie::{ request::HttpRequest, response::HttpResponse};
    ///
    /// fn get(request: &HttpRequest, response: &mut HttpResponse) {
    ///   let content_type = request.get_header("content-type");
    /// }
    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    /// Returns the method of the HTTP request
    pub(crate) fn get_method(&self) -> &HttpMethod {
        &self.request.get_method()
    }

    /// Returns the path of the HTTP request
    pub(crate) fn get_path_array(&self) -> &Vec<String> {
        &self.request.get_path_array()
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

impl Default for HttpRequest {
    fn default() -> Self {
        Self {
            request: RequestLine::new("GET", "/", "HTTP/1.1").expect(
                "Failed to create default RequestLine"
            ),
            headers: HashMap::new(),
            body: None,
        }
    }
}

impl Debug for HttpRequest {
    fn fmt(&self, f: &mut Formatter) -> fResult {
        let headers = self.headers
            .iter()
            .fold(String::new(), |acc, (k, v)| format!("{acc}{k}: {v}\r\n"));

        let body = match &self.body {
            Some(body) => format!("{:?}", body),
            None => String::new(),
        };

        write!(f, "Request Line: {}\r\n Headers: {}\r\n Body: {}", self.request, headers, body)
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

#[derive(Debug)]
pub struct ParseHttpRequestError;

impl Display for ParseHttpRequestError {
    fn fmt(&self, f: &mut Formatter) -> fResult {
        write!(f, "Failed to parse HTTP request")
    }
}
