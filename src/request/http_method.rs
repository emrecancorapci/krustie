//! HTTP method module
//!
//! This module contains the `HttpMethod` enum and `ParseHttpMethodError` error.

use std::fmt::{ Display, Error, Formatter, Result as fResult };

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
/// An enum that represents an HTTP method
///
/// The HTTP method is used to indicate the desired action to be performed for a given resource.
///
/// # Supported methods
///
/// - GET
/// - POST
/// - PUT
/// - PATCH
///
pub enum HttpMethod {
    /// GET method is used to request data from a specified resource
    GET,
    /// POST method is used to submit data to be processed to a specified resource
    POST,
    /// PUT method is used to update data to a specified resource
    PUT,
    /// PATCH method is used to apply partial modifications to a resource
    PATCH,
    /// DELETE method is used to delete a specified resource
    DELETE,
    // CONNTECT,
    // HEAD,
    // OPTIONS,
    // TRACE,
}

impl Default for HttpMethod {
    fn default() -> Self {
        Self::GET
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
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
    /// # Errors
    ///
    /// Returns an error if the string is not a valid HTTP method
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::HttpMethod;
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

/// Error for parsing an HTTP method
#[derive(Debug)]
pub struct ParseHttpMethodError;

impl Display for ParseHttpMethodError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        write!(f, "Invalid method for HTTP request")
    }
}
