use std::fmt::{self, Display, Formatter};

use super::StatusCode;

impl StatusCode {
    pub(super) fn get_message(&self) -> &str {
        match self {
            Self::Ok => "OK",
            Self::Created => "Created",
            Self::Accepted => "Accepted",
            Self::NoContent => "No Content",
            Self::BadRequest => "Bad Request",
            Self::Unauthorized => "Unauthorized",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
            Self::MethodNotAllowed => "Method Not Allowed",
            Self::RequestTimeout => "Request Timeout",
            Self::LengthRequired => "Length Required",
            Self::UnsupportedMediaType => "Unsupported Media Type",
            Self::IAmATeapot => "I'm A Teapot",
            Self::InternalServerError => "Internal Server Error",
            Self::NotImplemented => "Not Implemented",
            Self::ServiceUnavailable => "Service Unavailable",
            Self::GatewayTimeout => "Gateway Timeout",
            Self::HttpVersionNotSupported => "HTTP Version Not Supported",
        }
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        Self::Ok
    }
}

impl TryFrom<&u16> for StatusCode {
    type Error = ParseStatusCodeError;
    /// Converts a u16 to a StatusCode
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::response::StatusCode;
    ///
    /// match StatusCode::try_from(&200) {
    ///   Ok(status_code) => assert_eq!(status_code, StatusCode::Ok),
    ///   Err(_) => panic!("Invalid status code"),
    /// }
    /// match StatusCode::try_from(&418) {
    ///   Ok(status_code) => assert_eq!(status_code, StatusCode::IAmATeapot),
    ///   Err(_) => panic!("Invalid status code"),
    /// }
    /// ```
    fn try_from(code: &u16) -> Result<Self, Self::Error> {
        match code {
            200 => Ok(Self::Ok),
            201 => Ok(Self::Created),
            202 => Ok(Self::Accepted),
            204 => Ok(Self::NoContent),
            400 => Ok(Self::BadRequest),
            401 => Ok(Self::Unauthorized),
            403 => Ok(Self::Forbidden),
            404 => Ok(Self::NotFound),
            405 => Ok(Self::MethodNotAllowed),
            408 => Ok(Self::RequestTimeout),
            411 => Ok(Self::LengthRequired),
            415 => Ok(Self::UnsupportedMediaType),
            418 => Ok(Self::IAmATeapot),
            500 => Ok(Self::InternalServerError),
            501 => Ok(Self::NotImplemented),
            503 => Ok(Self::ServiceUnavailable),
            504 => Ok(Self::GatewayTimeout),
            505 => Ok(Self::HttpVersionNotSupported),
            _ => Err(ParseStatusCodeError),
        }
    }
}

impl TryFrom<&str> for StatusCode {
    type Error = ParseStatusCodeError;
    /// Converts a string to a StatusCode
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::response::StatusCode;
    ///
    /// match StatusCode::try_from("200") {
    ///   Ok(status_code) => assert_eq!(status_code, StatusCode::Ok),
    ///   Err(_) => panic!("Invalid status code"),
    /// }
    /// match StatusCode::try_from("418") {
    ///    Ok(status_code) => assert_eq!(status_code, StatusCode::IAmATeapot),
    ///   Err(_) => panic!("Invalid status code"),
    /// }
    /// ```
    fn try_from(code: &str) -> Result<Self, Self::Error> {
        let num = code.parse().unwrap_or(0);
        Self::try_from(num)
    }
}

impl TryFrom<u16> for StatusCode {
    type Error = ParseStatusCodeError;
    /// Converts a u16 to a StatusCode
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::response::StatusCode;
    ///
    /// match StatusCode::try_from(200) {
    ///   Ok(status_code) => assert_eq!(status_code, StatusCode::Ok),
    ///   Err(_) => panic!("Invalid status code"),
    /// }
    /// match StatusCode::try_from(418) {
    ///   Ok(status_code) => assert_eq!(status_code, StatusCode::IAmATeapot),
    ///   Err(_) => panic!("Invalid status code"),
    /// }
    ///
    /// ```
    fn try_from(code: u16) -> Result<Self, Self::Error> {
        Self::try_from(&code)
    }
}

impl From<&StatusCode> for u16 {
    /// Converts a StatusCode to a u16
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::response::StatusCode;
    ///
    /// let status_code_200 = StatusCode::Ok;
    /// let status_code_418 = StatusCode::IAmATeapot;
    ///
    /// assert_eq!(u16::from(&status_code_200), 200);
    /// assert_eq!(u16::from(&status_code_418), 418);
    /// ```
    fn from(code: &StatusCode) -> u16 {
        *code as u16
    }
}

impl Display for StatusCode {
    /// Converts a StatusCode to a string
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::response::StatusCode;
    ///
    /// let status_code_200 = StatusCode::Ok;
    /// let status_code_418 = StatusCode::IAmATeapot;
    ///
    /// assert_eq!(status_code_200.to_string(), "200");
    /// assert_eq!(status_code_418.to_string(), "418");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", *self as u16)
    }
}

#[derive(Debug)]
pub struct ParseStatusCodeError;

impl Display for ParseStatusCodeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Invalid status code for HTTP response")
    }
}