use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    net::SocketAddr,
    str,
};

use async_trait::async_trait;

use crate::http::core::{HttpMethod, HttpRequest};

const MAX_HEADER: usize = 100;

pub struct Request {
    path: String,
    method: HttpMethod,
    http_version: String,
    params: HashMap<String, String>,
    headers: HashMap<String, String>,
    queries: HashMap<String, String>,
    cookies: HashMap<String, String>,
    body: Vec<u8>,
    peer_addr: SocketAddr,
}

impl TryFrom<&str> for HttpMethod {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "HEAD" => Ok(HttpMethod::HEAD),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "CONNECT" => Ok(HttpMethod::CONNECT),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "TRACE" => Ok(HttpMethod::TRACE),
            "PATCH" => Ok(HttpMethod::PATCH),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                "Not a valid http method!",
            )),
        }
    }
}

#[async_trait]
impl HttpRequest for Request {
    async fn new(buffer: &[u8], addr: SocketAddr) -> Result<Self, std::io::Error> {
        let (request_headers, body) = split_request(buffer);

        let request_line: Vec<&str> = request_headers[0].split(' ').collect();

        if request_line.len() != 3 || !request_line[2].starts_with("HTTP/") {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Request line is not valid.",
            ));
        }

        let (method, uri, version) = (
            HttpMethod::try_from(request_line[0].trim())?,
            request_line[1].trim(),
            request_line[2].trim(),
        );

        if version != "HTTP/1.1" {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "HTTP version not supported.".to_string(),
            ));
        }

        let headers: HashMap<String, String> = request_headers
            .iter()
            .skip(1)
            .take(MAX_HEADER)
            .filter_map(|line: &&str| {
                let header_line: Vec<&str> = line.split(':').collect();

                if header_line.len() == 2 {
                    let key = header_line[0].trim().to_lowercase().to_string();
                    let value = header_line[1].trim().to_string();

                    Some((key, value))
                } else {
                    None
                }
            })
            .collect();

        let content_length = headers
            .get("content-length")
            .and_then(|len| Some(len.trim().parse::<usize>()))
            .unwrap_or(Ok(0 as usize))
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err.to_string()))?;

        let queries = if let Some((_, queries)) = uri.split_once('?') {
            if queries.contains('?') {
                Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid uri. Too many question marks.",
                ))
            } else {
                Ok(queries.split('&').fold(HashMap::new(), |mut result, kvp| {
                    if let Some((k, v)) = kvp.split_once('=') {
                        result.insert(k.to_string(), v.to_string());
                    }

                    result
                }))
            }
        } else {
            Ok(HashMap::new())
        }?;

        if content_length == 0 {
            return Ok(Self {
                path: String::from(uri),
                method,
                http_version: String::from(version),
                params: HashMap::new(),
                headers,
                queries,
                cookies: HashMap::new(),
                body: Vec::new(),
                peer_addr: addr,
            });
        }

        if body.is_empty() {
            return Err(Error::new(std::io::ErrorKind::NotFound, "Body is empty."));
        }

        return Ok(Self {
            path: String::from(uri),
            method,
            http_version: String::from(version),
            params: HashMap::new(),
            headers,
            queries,
            cookies: HashMap::new(),
            body: body.to_vec(),
            peer_addr: addr,
        });
    }

    fn get_method(&self) -> &HttpMethod {
        &self.method
    }

    fn get_path(&self) -> &str {
        &self.path
    }

    fn get_version(&self) -> &str {
        &self.http_version
    }

    fn get_body(&self) -> &[u8] {
        &self.body
    }

    fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    fn get_header(&self, k: &str) -> Option<&str> {
        self.headers.get(k).and_then(|k| Some(k.as_str()))
    }

    fn get_query_params(&self) -> &HashMap<String, String> {
        &self.queries
    }

    fn get_query_param(&self, k: &str) -> Option<&str> {
        self.queries.get(k).and_then(|k| Some(k.as_str()))
    }

    fn get_param(&self, k: &str) -> Option<&str> {
        self.params.get(k).and_then(|k| Some(k.as_str()))
    }

    fn get_cookies(&self) -> &HashMap<String, String> {
        &self.cookies
    }

    fn get_cookie(&self, k: &str) -> Option<&str> {
        self.cookies.get(k).and_then(|k| Some(k.as_str()))
    }

    fn get_content_type(&self) -> Option<&str> {
        self.headers
            .get("content-type")
            .and_then(|k| Some(k.as_str()))
    }

    fn get_user_agent(&self) -> Option<&str> {
        self.headers
            .get("user-agent")
            .and_then(|k| Some(k.as_str()))
    }

    fn accepts(&self, content_type: &str) -> bool {
        match self.headers.get("accepts").and_then(|k| Some(k.as_str())) {
            Some(accepts) if accepts.contains(content_type) => true,
            _ => false,
        }
    }

    async fn parse_form_data(&self) -> HashMap<String, String> {
        todo!()
    }

    fn is_multipart(&self) -> bool {
        todo!()
    }

    fn get_multipart_fields(&self) -> HashMap<String, String> {
        todo!()
    }

    fn get_multipart_field(&self, _name: &str) -> Option<&str> {
        todo!()
    }

    fn get_multipart_files(&self) -> HashMap<String, Vec<u8>> {
        todo!()
    }

    fn get_peer_addr(&self) -> &SocketAddr {
        &self.peer_addr
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
