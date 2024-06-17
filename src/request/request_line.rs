use std::fmt::{ Display, Formatter, Result as fResult };

use super::HttpMethod;

pub struct RequestLine {
    method: HttpMethod,
    path: String,
    version: String,
    path_array: Vec<String>,
}

impl RequestLine {
    pub fn new(
        method: &str,
        path: &str,
        version: &str
    ) -> Result<RequestLine, ParseRequestLineError> {
        let path_array: Vec<String> = path[1..]
            .split('/')
            .map(|str| str.to_string())
            .collect();
        match HttpMethod::try_from(method) {
            Ok(method) => {
                Ok(RequestLine {
                    method,
                    path: path.to_string(),
                    version: version.to_string(),
                    path_array,
                })
            }
            Err(_) => {
                return Err(ParseRequestLineError);
            }
        }
    }

    pub fn get_method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_path_array(&self) -> &Vec<String> {
        &self.path_array
    }
}

impl Display for RequestLine {
    fn fmt(&self, f: &mut Formatter) -> fResult {
        write!(f, "{} {} {}", self.method, self.path, self.version)
    }
}

impl TryFrom<&str> for RequestLine {
    type Error = ParseRequestLineError;
    fn try_from(request_line: &str) -> Result<Self, Self::Error> {
        let request_line: Vec<&str> = request_line.split(' ').collect();

        if request_line.len() < 3 {
            return Err(ParseRequestLineError);
        }

        match RequestLine::new(request_line[0], request_line[1], request_line[2]) {
            Ok(request_line) => Ok(request_line),
            Err(_) => Err(ParseRequestLineError),
        }
    }
}

#[derive(Debug)]
pub struct ParseRequestLineError;

impl Display for ParseRequestLineError {
    fn fmt(&self, f: &mut Formatter) -> fResult {
        write!(f, "invalid request line")
    }
}
