use std::collections::HashMap;

use self::request_line::RequestLine;

pub mod request_line;

pub struct HttpRequest<'a> {
    pub request: RequestLine<'a>,
    pub headers: HashMap<String, String>,
    pub body: &'a str,
}

impl<'a> HttpRequest<'a> {
    pub fn from(http_request: &'a Vec<String>, body: &'a str) -> HttpRequest<'a> {
        let request = (
            {
                match http_request.first() {
                    Some(request_line) => { RequestLine::from_string(request_line) }
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

    pub fn debug(&self) {
        let headers = self.headers
            .iter()
            .fold(String::new(), |acc, (k, v)| format!("{acc}{k}: {v}\r\n"));

        println!(
            "HttpRequest Line: {request}\r\n Headers: {headers}\r\n Body: {body}",
            request = self.request.to_string(),
            body = self.body
        );
    }
}
