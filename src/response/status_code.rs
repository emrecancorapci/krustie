use std::fmt::{self, Display, Formatter};

use super::StatusCode;

impl StatusCode {
    pub fn get_message(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NoContent => "No Content",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::RequestTimeout => "Request Timeout",
            StatusCode::LengthRequired => "Length Required",
            StatusCode::UnsupportedMediaType => "Unsupported Media Type",
            StatusCode::IAmATeapot => "I'm A Teapot",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
            StatusCode::ServiceUnavailable => "Service Unavailable",
            StatusCode::GatewayTimeout => "Gateway Timeout",
            StatusCode::HttpVersionNotSupported => "HTTP Version Not Supported",
        }
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::Ok
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
            200 => Ok(StatusCode::Ok),
            201 => Ok(StatusCode::Created),
            202 => Ok(StatusCode::Accepted),
            204 => Ok(StatusCode::NoContent),
            400 => Ok(StatusCode::BadRequest),
            401 => Ok(StatusCode::Unauthorized),
            403 => Ok(StatusCode::Forbidden),
            404 => Ok(StatusCode::NotFound),
            405 => Ok(StatusCode::MethodNotAllowed),
            408 => Ok(StatusCode::RequestTimeout),
            411 => Ok(StatusCode::LengthRequired),
            415 => Ok(StatusCode::UnsupportedMediaType),
            418 => Ok(StatusCode::IAmATeapot),
            500 => Ok(StatusCode::InternalServerError),
            501 => Ok(StatusCode::NotImplemented),
            503 => Ok(StatusCode::ServiceUnavailable),
            504 => Ok(StatusCode::GatewayTimeout),
            505 => Ok(StatusCode::HttpVersionNotSupported),
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
        StatusCode::try_from(num)
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
        StatusCode::try_from(&code)
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
        match code {
            StatusCode::Ok => 200,
            StatusCode::Created => 201,
            StatusCode::Accepted => 202,
            StatusCode::NoContent => 204,
            StatusCode::BadRequest => 400,
            StatusCode::Unauthorized => 401,
            StatusCode::Forbidden => 403,
            StatusCode::NotFound => 404,
            StatusCode::MethodNotAllowed => 405,
            StatusCode::RequestTimeout => 408,
            StatusCode::LengthRequired => 411,
            StatusCode::UnsupportedMediaType => 415,
            StatusCode::IAmATeapot => 418,
            StatusCode::InternalServerError => 500,
            StatusCode::NotImplemented => 501,
            StatusCode::ServiceUnavailable => 503,
            StatusCode::GatewayTimeout => 504,
            StatusCode::HttpVersionNotSupported => 505,
        }
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
        let str = match self {
            StatusCode::Ok => "200",
            StatusCode::Created => "201",
            StatusCode::Accepted => "202",
            StatusCode::NoContent => "204",
            StatusCode::BadRequest => "400",
            StatusCode::Unauthorized => "401",
            StatusCode::Forbidden => "403",
            StatusCode::NotFound => "404",
            StatusCode::MethodNotAllowed => "405",
            StatusCode::RequestTimeout => "408",
            StatusCode::LengthRequired => "411",
            StatusCode::UnsupportedMediaType => "415",
            StatusCode::IAmATeapot => "418",
            StatusCode::InternalServerError => "500",
            StatusCode::NotImplemented => "501",
            StatusCode::ServiceUnavailable => "503",
            StatusCode::GatewayTimeout => "504",
            StatusCode::HttpVersionNotSupported => "505",
        };

        write!(f, "{}", str)
    }
}

#[derive(Debug)]
pub struct ParseStatusCodeError;

impl Display for ParseStatusCodeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Invalid status code for HTTP response")
    }
}