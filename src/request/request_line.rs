use std::fmt::{Display, Formatter, Result as fResult};

use super::http_method::HttpMethod;

#[derive(Clone)]
pub(crate) struct RequestLine {
    method: HttpMethod,
    uri: String,
    version: String,
    path_array: Vec<String>,
}

impl RequestLine {
    pub(super) fn new(
        method: &str,
        uri: &str,
        version: &str,
    ) -> Result<Self, ParseRequestLineError> {
        match HttpMethod::try_from(method) {
            Ok(method) => Ok(Self {
                method,
                uri: uri.to_string(),
                version: version.to_string(),
                path_array: Self::uri_parser(uri),
            }),
            Err(_) => {
                return Err(ParseRequestLineError);
            }
        }
    }

    fn uri_parser(uri: &str) -> Vec<String> {
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
        &self.path_array
    }

    pub(super) fn get_version(&self) -> &String {
        &self.version
    }

    pub(super) fn get_path(&self) -> &str {
        &self.uri.as_str()
    }

    pub(crate) fn set_method(&mut self, method: HttpMethod) {
        self.method = method;
    }

    pub(crate) fn set_uri(&mut self, uri: &str) {
        self.uri = String::from(uri);
        self.path_array = Self::uri_parser(uri);
    }

    pub(crate) fn set_version(&mut self, version: &str) {
        self.version = String::from(version.trim());
    }
}

impl Display for RequestLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        write!(f, "{} {} {}", self.method, self.uri, self.version)
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

        let (method, uri, version) = (
            request_line[0].trim(),
            request_line[1].trim(),
            request_line[2].trim(),
        );

        match Self::new(method, uri, version) {
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
