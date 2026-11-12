use core::str;
use std::{
    collections::HashMap, io::{Error, ErrorKind}, str::Utf8Error,
};

use tokio::{io::AsyncReadExt, net::TcpStream};

use super::{ParseHttpRequestError, Request, RequestBody, request_line::RequestLine};

const MAX_HEADER: usize = 100;

impl Request {
    /// Parses a TcpStream into Request
    pub(crate) async fn parse(stream: &mut TcpStream) -> Result<Self, Error> {
        let peer_addr = stream.peer_addr()?;
        let buffer: &mut [u8] = &mut [0; 1024];

        let stream_length = stream.read(buffer).await?;

        let (http_request, body) = Self::split_request(&buffer[..stream_length])?;

        let request_line_str = http_request.get(0).ok_or(Error::new(
            ErrorKind::InvalidInput,
            "Error while parsing request line".to_string(),
        ))?;

        let request_line = RequestLine::try_from(*request_line_str).map_err(|_err| {
            Error::new(
                ErrorKind::InvalidInput,
                "Error while parsing request line".to_string(),
            )
        })?;

        if request_line.get_version() != "HTTP/1.1" {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid HTTP version".to_string(),
            ));
        }

        let headers: HashMap<String, String> = http_request
            .iter()
            .skip(1)
            .take(MAX_HEADER)
            .filter_map(Request::header_parser())
            .collect();

        let content_length = Self::parse_length(&http_request).unwrap_or(0);

        let queries = Self::parse_queries(&request_line.get_path_array());

        if content_length == 0 {
            return Ok(Request {
                request: request_line,
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

    pub(crate) fn parse_queries(path_array: &Vec<String>) -> HashMap<String, String> {
        let path_last = path_array.last();

        return match path_last {
            Some(last) if !last.is_empty() && last.contains('?') => {
                Self::parse_query_uri(last).unwrap_or(HashMap::new())
            }
            _ => HashMap::new(),
        };
    }

    fn parse_query_uri(uri_last: &str) -> Result<HashMap<String, String>, Error> {
        let query_params = uri_last.split('?').skip(1).collect::<Vec<&str>>();

        if query_params.len() != 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid query usage".to_string(),
            ));
        }

        return Ok(query_params[0]
            .split('&')
            .fold(HashMap::new(), |mut queries, kvp| {
                if let Some((k, v)) = kvp.split_once('=') {
                    queries.insert(k.to_string(), v.to_string());
                }

                return queries;
            }));
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

    fn split_request(vec: &[u8]) -> Result<(Vec<&str>, &[u8]), Error> {
        let head_end = vec
            .windows(4)
            .position(|w| w == b"\r\n\r\n")
            .map(|pos| pos + 4)
            .or_else(|| vec.windows(2).position(|w| w == b"\n\n").map(|pos| pos + 2))
            .unwrap_or(vec.len());

        let http_request: Vec<&str> = vec[..head_end]
            .split(|&x| x == b'\n')
            .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
            .filter(|line| !line.is_empty())
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, Utf8Error>>()
            .map_err(|_| Error::new(ErrorKind::InvalidData, ParseHttpRequestError.to_string()))?;

        Ok((http_request, &vec[head_end..]))
    }
}
