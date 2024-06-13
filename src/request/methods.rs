use std::{ collections::HashMap, fmt::Debug, net::IpAddr };

use super::{ request_line::RequestLine, HttpRequest };

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
    pub fn new(http_request: &'a Vec<String>, body: &'a str, ip: IpAddr) -> HttpRequest<'a> {
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
            ip,
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
            body: "",
            ip: IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
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
