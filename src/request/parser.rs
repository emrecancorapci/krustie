use std::{
    collections::HashMap,
    io::{ BufRead, BufReader, Error, ErrorKind, Read },
    net::TcpStream,
};

use super::{ request_line::RequestLine, Request, ParseHttpRequestError, RequestBody };

const MAX_HEADER: usize = 100;

impl Request {
    /// Parses a TcpStream into Request
    pub(crate) fn parse(mut stream: &TcpStream) -> Result<Self, Error> {
        let peer_addr = stream.peer_addr()?;

        let mut buf_reader = BufReader::new(&mut stream);
        let mut http_request = Vec::new();
        let mut queries = HashMap::new();

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
            return Err(
                Error::new(ErrorKind::InvalidInput, "Error while parsing request line".to_string())
            );
        }

        let request_line = request_line.unwrap();

        if request_line.get_version() != "HTTP/1.1" {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid HTTP version".to_string()));
        }

        if
            request_line
                .get_path_array()
                .last()
                .is_some_and(|last| last.contains('?'))
        {
            let last = request_line.get_path_array().last().unwrap();

            let maybe_queries = last.split('?').skip(1).collect::<Vec<&str>>();

            if maybe_queries.len() != 1 {
                return Err(Error::new(ErrorKind::InvalidInput, "Invalid query usage".to_string()));
            }

            maybe_queries[0].split('&').for_each(|kvp| {
                if let Some((k, v)) = kvp.split_once('=') {
                    queries.insert(k.to_string(), v.to_string());
                }
            });
        }

        let headers: HashMap<String, String> = http_request
            .iter()
            .skip(1)
            .take(MAX_HEADER)
            .filter_map(Request::header_parser())
            .collect();

        let content_length = Self::parse_length(&http_request).unwrap_or(0);

        if content_length == 0 {
            return Ok(Request {
                request: RequestLine::try_from(http_request[0].as_str()).unwrap(),
                headers,
                queries,
                peer_addr,
                body: RequestBody::None,
            });
        }

        let mut body = Vec::with_capacity(content_length);

        buf_reader.take(content_length as u64).read_to_end(&mut body)?;

        let body: RequestBody = Self::parse_body(body, &headers)?;

        Ok(Request {
            request: request_line,
            headers,
            queries,
            peer_addr,
            body,
        })
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

    fn parse_length(headers: &[String]) -> Option<usize> {
        for line in headers.iter() {
            if line.starts_with("Content-Length") {
                match line.find(':') {
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

    fn parse_body(body: Vec<u8>, headers: &HashMap<String, String>) -> Result<RequestBody, Error> {
        if body.is_empty() {
            return Err(Error::new(std::io::ErrorKind::NotFound, "Body is empty."));
        }

        match headers.get("content-type") {
            Some(content_type) => {
                return RequestBody::parse(body, content_type);
            }
            None => {
                return Err(Error::new(ErrorKind::NotFound, ParseHttpRequestError.to_string()));
            }
        }
    }
}
