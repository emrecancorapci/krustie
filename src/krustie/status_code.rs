use std::io::Error;

use crate::http::core::HttpStatusCode;

/// Represents the status code of an HTTP response
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum StatusCode {
    NotSet = 0,
    /// 200 OK
    Ok = 200,
    /// 201 Created
    Created = 201,
    /// 202 Accepted
    Accepted = 202,
    /// 204 No Content
    NoContent = 204,
    /// 400 Bad Request
    BadRequest = 400,
    /// 401 Unauthorized
    Unauthorized = 401,
    /// 403 Forbidden
    Forbidden = 403,
    /// 404 Not Found
    NotFound = 404,
    /// 405 Method Not Allowed
    MethodNotAllowed = 405,
    /// 408 Request Timeout
    RequestTimeout = 408,
    /// 411 Length Required
    LengthRequired = 411,
    /// 415 Unsupported Media Type
    UnsupportedMediaType = 415,
    /// 418 I'm A Teapot
    IAmATeapot = 418,
    /// 249 Too Many Requests
    TooManyRequests = 429,
    /// 500 Internal Server Error
    InternalServerError = 500,
    /// 501 Not Implemented
    NotImplemented = 501,
    /// 503 Service Unavailable
    ServiceUnavailable = 503,
    /// 504 Gateway Timeout
    GatewayTimeout = 504,
    /// 505 HTTP Version Not Supported
    HttpVersionNotSupported = 505,
}

impl HttpStatusCode for StatusCode {
    fn code(&self) -> u16 {
        *self as u16
    }
    fn msg(&self) -> &str {
        use StatusCode::*;

        match self {
            Ok => "OK",
            Created => "Created",
            Accepted => "Accepted",
            NoContent => "No Content",
            BadRequest => "Bad Request",
            Unauthorized => "Unauthorized",
            Forbidden => "Forbidden",
            NotFound => "Not Found",
            MethodNotAllowed => "Method Not Allowed",
            RequestTimeout => "Request Timeout",
            LengthRequired => "Length Required",
            UnsupportedMediaType => "Unsupported Media Type",
            IAmATeapot => "I'm A Teapot",
            TooManyRequests => "Too Many Requests",
            InternalServerError => "Internal Server Error",
            NotImplemented => "Not Implemented",
            ServiceUnavailable => "Service Unavailable",
            GatewayTimeout => "Gateway Timeout",
            HttpVersionNotSupported => "HTTP Version Not Supported",
            NotSet => panic!("Status code not set"),
        }
    }

    fn is_err(&self) -> bool {
        self.code() >= 400
    }
}

impl TryFrom<u16> for StatusCode {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(StatusCode::NotSet),
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
            429 => Ok(StatusCode::TooManyRequests),
            500 => Ok(StatusCode::InternalServerError),
            501 => Ok(StatusCode::NotImplemented),
            503 => Ok(StatusCode::ServiceUnavailable),
            504 => Ok(StatusCode::GatewayTimeout),
            505 => Ok(StatusCode::HttpVersionNotSupported),
            _ => Err(Error::new(
                std::io::ErrorKind::NotFound,
                "The value is not a status code or not supported",
            )),
        }
    }
}
