use core::str;
use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Read},
    net::TcpStream,
};

use super::{request_line::RequestLine, ParseHttpRequestError, Request, RequestBody};

const MAX_HEADER: usize = 100;

impl Request {
    /// Parses a TcpStream into Request
    pub(crate) fn parse(mut stream: &TcpStream) -> Result<Self, Error> {
        let peer_addr = stream.peer_addr()?;
        let buffer: &mut [u8] = &mut [0; 1024];

        let stream_length = stream.read(buffer)?;

        let (http_request, body) = Self::split_request(&buffer[..stream_length]);

        let request_line = RequestLine::try_from(http_request[0]);

        if request_line.is_err() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Error while parsing request line".to_string(),
            ));
        }

        let request_line = request_line.unwrap();

        if request_line.get_version() != "HTTP/1.1" {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid HTTP version".to_string(),
            ));
        }

        let mut queries = HashMap::new();

        if request_line
            .get_path_array()
            .last()
            .is_some_and(|last| last.contains('?'))
        {
            let last = request_line.get_path_array().last().unwrap();

            let query_params = last.split('?').skip(1).collect::<Vec<&str>>();

            if query_params.len() != 1 {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid query usage".to_string(),
                ));
            }

            query_params[0].split('&').for_each(|kvp| {
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
                request: RequestLine::try_from(http_request[0]).unwrap(),
                headers,
                queries,
                peer_addr,
                params: HashMap::new(),
                body: RequestBody::None,
            });
        }

        let body: RequestBody = Self::parse_body(body, &headers)?;

        Ok(Request {
            request: request_line,
            headers,
            queries,
            params: HashMap::new(),
            peer_addr,
            body,
        })
    }

    fn header_parser() -> impl Fn(&&str) -> Option<(String, String)> {
        |line: &&str| {
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

    fn parse_length(headers: &[&str]) -> Option<usize> {
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

    fn parse_body(body: &[u8], headers: &HashMap<String, String>) -> Result<RequestBody, Error> {
        if body.is_empty() {
            return Err(Error::new(std::io::ErrorKind::NotFound, "Body is empty."));
        }

        match headers.get("content-type") {
            Some(content_type) => {
                return RequestBody::parse(body, content_type);
            }
            None => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    ParseHttpRequestError.to_string(),
                ));
            }
        }
    }

    fn split_request(vec: &[u8]) -> (Vec<&str>, &[u8]) {
        let mut split = vec.split(|&x| x == b'\n');

        let mut http_request = Vec::new();

        while let Some(line) = split.next() {
            if line.is_empty() {
                break;
            }

            http_request.push(str::from_utf8(line).unwrap());
        }

        let body = split.next().unwrap_or(&[]);

        (http_request, body)
    }
}
