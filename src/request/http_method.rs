use std::fmt::{ Display, Error, Formatter };

use super::HttpMethod;

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

impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::GET
    }
}

impl From<&str> for HttpMethod {
    /// Converts a string to an HttpMethod
    ///
    /// If the string is not a valid HTTP method, it defaults to GET
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::request::HttpMethod;
    ///
    /// fn main() {
    ///    let method = HttpMethod::from("GET");
    /// }
    fn from(method: &str) -> Self {
        let method = method.to_uppercase();
        match method.as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "PATCH" => HttpMethod::PATCH,
            "DELETE" => HttpMethod::DELETE,
            _ => HttpMethod::GET,
        }
    }
}
