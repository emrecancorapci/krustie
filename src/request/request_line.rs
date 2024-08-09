use std::fmt::{Display, Formatter, Result as fResult};

use super::http_method::HttpMethod;

pub(crate) struct RequestLine {
    method: HttpMethod,
    path: String,
    version: String,
    path_array: Vec<String>,
}

impl RequestLine {
    pub(super) fn new(
        method: &str,
        path: &str,
        version: &str,
    ) -> Result<Self, ParseRequestLineError> {
        let path_array: Vec<String> = path[1..].split('/').map(|str| str.to_string()).collect();
        match HttpMethod::try_from(method) {
            Ok(method) => Ok(Self {
                method,
                path: path.to_string(),
                version: version.to_string(),
                path_array,
            }),
            Err(_) => {
                return Err(ParseRequestLineError);
            }
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

    pub(super) fn get_path(&self) -> &String {
        &self.path
    }
}

impl Display for RequestLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        write!(f, "{} {} {}", self.method, self.path, self.version)
    }
}

impl TryFrom<&str> for RequestLine {
    type Error = ParseRequestLineError;
    fn try_from(request_line: &str) -> Result<Self, Self::Error> {
        let request_line: Vec<&str> = request_line.split(' ').collect();

        if request_line.len() != 3 {
            return Err(ParseRequestLineError);
        }

        if !HttpMethod::is_valid(request_line[0])
            || !request_line[1].starts_with('/')
            || !request_line[2].starts_with("HTTP/")
        {
            return Err(ParseRequestLineError);
        }

        match Self::new(request_line[0], request_line[1], request_line[2]) {
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
