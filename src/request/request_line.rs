use std::fmt::{Display, Formatter, Result as fResult};

use super::http_method::HttpMethod;

#[derive(Clone)]
pub(crate) struct RequestLine {
    method: HttpMethod,
    request_uri: String,
    version: String,
    request_uri_array: Vec<String>,
}

impl RequestLine {
    pub(super) fn new(
        method: &str,
        request_uri: &str,
        version: &str,
    ) -> Result<Self, ParseRequestLineError> {
        println!("{:?}", Self::request_uri_parser(request_uri));

        match HttpMethod::try_from(method) {
            Ok(method) => Ok(Self {
                method,
                request_uri: request_uri.to_string(),
                version: version.to_string(),
                request_uri_array: Self::request_uri_parser(request_uri),
            }),
            Err(_) => {
                return Err(ParseRequestLineError);
            }
        }
    }

    fn request_uri_parser(uri: &str) -> Vec<String> {
        if uri.starts_with('/') {
            uri.split('/').skip(1).map(|str| str.to_string()).collect()
        } else if uri.starts_with("http") {
            uri.split('/').skip(3).map(|str| str.to_string()).collect()
        } else {
            uri.split('/').skip(1).map(|str| str.to_string()).collect()
        }
    }

    pub(super) fn get_method(&self) -> &HttpMethod {
        &self.method
    }

    pub(super) fn get_path_array(&self) -> &Vec<String> {
        &self.request_uri_array
    }

    pub(super) fn get_version(&self) -> &String {
        &self.version
    }

    pub(super) fn get_path(&self) -> &str {
        &self.request_uri.as_str()
    }
}

impl Display for RequestLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        write!(f, "{} {} {}", self.method, self.request_uri, self.version)
    }
}

impl TryFrom<&str> for RequestLine {
    type Error = ParseRequestLineError;
    fn try_from(request_line: &str) -> Result<Self, Self::Error> {
        let request_line: Vec<&str> = request_line.split(' ').collect();

        if request_line.len() != 3
            || !HttpMethod::is_valid(request_line[0])
            || !request_line[2].starts_with("HTTP/")
        {
            return Err(ParseRequestLineError);
        }

        let (method, request_uri, version) = (request_line[0], request_line[1], request_line[2]);

        match Self::new(method, request_uri, version) {
            Ok(request_line) => Ok(request_line),
            Err(_) => Err(ParseRequestLineError),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ParseRequestLineError;

impl Display for ParseRequestLineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        write!(f, "invalid request line")
    }
}
