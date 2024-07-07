use std::{ collections::HashMap, io::{ BufRead, BufReader, Read }, net::TcpStream };

use super::{ request_line::RequestLine, BodyType, Request, ParseHttpRequestError };

impl Request {
    /// Parses a TcpStream into Request
    pub(crate) fn parse(mut stream: &TcpStream) -> Result<Self, String> {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut http_request = Vec::new();

        // Don't touch this. It's too sensitive :(((.
        for line_result in buf_reader.by_ref().lines() {
            let line = line_result.unwrap();
            if line.is_empty() {
                break;
            }
            http_request.push(line);
        }

        let request_line = RequestLine::try_from(http_request[0].as_str());

        if request_line.is_err() {
            return Err("Error while parsing request line".to_string());
        }

        let headers: HashMap<String, String> = http_request
            .iter()
            .skip(1)
            .filter_map(Request::header_parser())
            .collect();

        let content_length = parse_length(&http_request);

        if content_length.is_none() || content_length.unwrap() == 0 {
            return Ok(Request {
                request: RequestLine::try_from(http_request[0].as_str()).unwrap(),
                headers,
                body: BodyType::None,
                locals: HashMap::new(),
            });
        }

        let content_length = content_length.unwrap();

        let mut body = Vec::with_capacity(content_length);

        if
            buf_reader
                .take(content_length as u64)
                .read_to_end(&mut body)
                .is_err()
        {
            return Err("Error while reading body".to_string());
        }

        let body = match parse_body(body, &headers) {
            Ok(value) => value,
            Err(value) => {
                return Err(value);
            }
        };

        Ok(Request {
            request: request_line.unwrap(),
            headers,
            body,
            locals: HashMap::new(),
        })
    }
}

fn parse_body(body: Vec<u8>, headers: &HashMap<String, String>) -> Result<BodyType, String> {
    if body.len() == 0 {
        return Err("Error while reading body".to_string());
    }

    match headers.get("content-type") {
        Some(content_type) => {
            return BodyType::parse(body, content_type);
        }
        None => {
            return Err(ParseHttpRequestError.to_string());
        }
    }
}

/// Gets the content length from the headers
fn parse_length(headers: &Vec<String>) -> Option<usize> {
    for line in headers.iter() {
        if line.starts_with("Content-Length") {
            match line.find(":") {
                Some(index) => {
                    return Some(line[index + 1..].trim().parse::<usize>().unwrap_or(0));
                }
                None => {
                    return None;
                }
            }
        }
    }
    return None;
}
