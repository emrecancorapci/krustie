use std::{ collections::HashMap, fmt::{ Debug, Formatter, Result as fResult } };
use self::request_line::RequestLine;

pub mod http_method;
mod request_line;
mod parser;

pub struct HttpRequest {
    pub request: RequestLine,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    /// Creates a new HttpRequest
    fn new(http_request: &Vec<String>, body: &str) -> HttpRequest {
        let request = (
            {
                match http_request.first() {
                    Some(request_line) => { RequestLine::try_from(request_line.as_str()) }
                    None => { todo!("Implement none handling") }
                }
            }
        ).expect("RequestLine not found");

        let headers = http_request
            .iter()
            .skip(1)
            .filter_map(HttpRequest::header_parser())
            .collect();

        let body = body.as_bytes().to_vec();

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

impl Default for HttpRequest {
    fn default() -> Self {
        HttpRequest {
            request: RequestLine::new("GET", "/", "HTTP/1.1").expect("Failed to create default RequestLine"),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}

impl Debug for HttpRequest {
    fn fmt(&self, f: &mut Formatter) -> fResult {
        let headers = self.headers
            .iter()
            .fold(String::new(), |acc, (k, v)| format!("{acc}{k}: {v}\r\n"));

        let body = self.body
            .iter()
            .map(|byte| *byte as char)
            .collect::<String>();

        write!(
            f,
            "HttpRequest Line: {}\r\n Headers: {}\r\n Body: {}",
            self.request.to_string(),
            headers,
            body
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
