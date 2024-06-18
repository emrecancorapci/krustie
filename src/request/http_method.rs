use std::fmt::{ Display, Error, Formatter, Result as fResult };

use super::HttpMethod;

impl Default for HttpMethod {
    fn default() -> Self {
        Self::GET
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::GET => write!(f, "GET"),
            Self::POST => write!(f, "POST"),
            Self::PUT => write!(f, "PUT"),
            Self::PATCH => write!(f, "PATCH"),
            Self::DELETE => write!(f, "DELETE"),
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
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
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
