use std::fmt::{ Display, Error, Formatter, Result as fResult };

use super::HttpMethod;

impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::GET
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::DELETE => write!(f, "DELETE"),
        }
    }
}

impl TryFrom<&str> for HttpMethod {
    type Error = ParseHttpMethodError;
    /// Converts a string to an HttpMethod
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::request::HttpMethod;
    ///
    /// fn main() {
    ///   match HttpMethod::try_from("GET") {
    ///     Ok(method) => assert_eq!(method, HttpMethod::GET),
    ///     Err(_) => panic!("Failed to parse HTTP method"),
    ///   }
    /// }
    /// ```
    fn try_from(method: &str) -> Result<Self, Self::Error> {
        let method = method.to_uppercase();
        match method.as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "PATCH" => Ok(HttpMethod::PATCH),
            "DELETE" => Ok(HttpMethod::DELETE),
            _ => Err(ParseHttpMethodError),
        }
    }
}

#[derive(Debug)]
pub struct ParseHttpMethodError;

impl Display for ParseHttpMethodError {
    fn fmt(&self, f: &mut Formatter) -> fResult {
        write!(f, "invalid method for HTTP request")
    }
}
