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

impl From<&str> for StatusCode {
    /// Converts a string to a StatusCode
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::response::StatusCode;
    ///
    /// let status_code_200 = StatusCode::from("200");
    /// let status_code_418 = StatusCode::from("418");
    ///
    /// assert_eq!(status_code_200, StatusCode::Ok);
    /// assert_eq!(status_code_418, StatusCode::IAmATeapot);
    /// ```
    fn from(code: &str) -> Self {
        match code {
            "200" => StatusCode::Ok,
            "201" => StatusCode::Created,
            "202" => StatusCode::Accepted,
            "204" => StatusCode::NoContent,
            "400" => StatusCode::BadRequest,
            "401" => StatusCode::Unauthorized,
            "403" => StatusCode::Forbidden,
            "404" => StatusCode::NotFound,
            "405" => StatusCode::MethodNotAllowed,
            "408" => StatusCode::RequestTimeout,
            "411" => StatusCode::LengthRequired,
            "415" => StatusCode::UnsupportedMediaType,
            "418" => StatusCode::IAmATeapot,
            "500" => StatusCode::InternalServerError,
            "501" => StatusCode::NotImplemented,
            "503" => StatusCode::ServiceUnavailable,
            "504" => StatusCode::GatewayTimeout,
            "505" => StatusCode::HttpVersionNotSupported,
            _ => StatusCode::Ok,
        }
    }
}

impl From<&u16> for StatusCode {
    /// Converts a u16 to a StatusCode
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::response::StatusCode;
    ///
    /// let status_code_200 = StatusCode::from(&200);
    /// let status_code_418 = StatusCode::from(&418);
    ///
    /// assert_eq!(status_code_200, StatusCode::Ok);
    /// assert_eq!(status_code_418, StatusCode::IAmATeapot);
    /// ```
    fn from(code: &u16) -> Self {
        match code {
            200 => StatusCode::Ok,
            201 => StatusCode::Created,
            202 => StatusCode::Accepted,
            204 => StatusCode::NoContent,
            400 => StatusCode::BadRequest,
            401 => StatusCode::Unauthorized,
            403 => StatusCode::Forbidden,
            404 => StatusCode::NotFound,
            405 => StatusCode::MethodNotAllowed,
            408 => StatusCode::RequestTimeout,
            411 => StatusCode::LengthRequired,
            415 => StatusCode::UnsupportedMediaType,
            418 => StatusCode::IAmATeapot,
            500 => StatusCode::InternalServerError,
            501 => StatusCode::NotImplemented,
            503 => StatusCode::ServiceUnavailable,
            504 => StatusCode::GatewayTimeout,
            505 => StatusCode::HttpVersionNotSupported,
            _ => StatusCode::Ok,
        }
    }
}

impl From<u16> for StatusCode {
    /// Converts a u16 to a StatusCode
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::response::StatusCode;
    ///
    /// let status_code_200 = StatusCode::from(200);
    /// let status_code_418 = StatusCode::from(418);
    ///
    /// assert_eq!(status_code_200, StatusCode::Ok);
    /// assert_eq!(status_code_418, StatusCode::IAmATeapot);
    /// ```
    fn from(code: u16) -> Self {
        match code {
            200 => StatusCode::Ok,
            201 => StatusCode::Created,
            202 => StatusCode::Accepted,
            204 => StatusCode::NoContent,
            400 => StatusCode::BadRequest,
            401 => StatusCode::Unauthorized,
            403 => StatusCode::Forbidden,
            404 => StatusCode::NotFound,
            405 => StatusCode::MethodNotAllowed,
            408 => StatusCode::RequestTimeout,
            411 => StatusCode::LengthRequired,
            415 => StatusCode::UnsupportedMediaType,
            418 => StatusCode::IAmATeapot,
            500 => StatusCode::InternalServerError,
            501 => StatusCode::NotImplemented,
            503 => StatusCode::ServiceUnavailable,
            504 => StatusCode::GatewayTimeout,
            505 => StatusCode::HttpVersionNotSupported,
            _ => StatusCode::Ok,
        }
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

// TODO: Implement From<StatusCode> for &str
// Gives this error:
// let status_code_200_str = str::from(status_code_200);
//                           ^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time

// impl From<StatusCode> for &str {
//     /// Converts a StatusCode to a string
//     ///
//     /// # Example
//     ///
//     /// ```rust
//     /// use krustie::response::StatusCode;
//     ///
//     /// let status_code_200 = StatusCode::Ok;
//     /// let status_code_418 = StatusCode::IAmATeapot;
//     ///
//     /// assert_eq!(str::from(status_code_200), "200");
//     /// assert_eq!(str::from(status_code_418), "418");
//     /// ```
//     fn from(code: StatusCode) -> &'static str {
//         match code {
//             StatusCode::Ok => "200",
//             StatusCode::Created => "201",
//             StatusCode::BadRequest => "400",
//             StatusCode::NotFound => "404",
//             StatusCode::MethodNotAllowed => "405",
//             StatusCode::IAmATeapot => "418",
//             StatusCode::InternalServerError => "500",
//         }
//     }
// }

impl ToString for StatusCode {
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
    fn to_string(&self) -> String {
        match self {
            StatusCode::Ok => "200".to_string(),
            StatusCode::Created => "201".to_string(),
            StatusCode::Accepted => "202".to_string(),
            StatusCode::NoContent => "204".to_string(),
            StatusCode::BadRequest => "400".to_string(),
            StatusCode::Unauthorized => "401".to_string(),
            StatusCode::Forbidden => "403".to_string(),
            StatusCode::NotFound => "404".to_string(),
            StatusCode::MethodNotAllowed => "405".to_string(),
            StatusCode::RequestTimeout => "408".to_string(),
            StatusCode::LengthRequired => "411".to_string(),
            StatusCode::UnsupportedMediaType => "415".to_string(),
            StatusCode::IAmATeapot => "418".to_string(),
            StatusCode::InternalServerError => "500".to_string(),
            StatusCode::NotImplemented => "501".to_string(),
            StatusCode::ServiceUnavailable => "503".to_string(),
            StatusCode::GatewayTimeout => "504".to_string(),
            StatusCode::HttpVersionNotSupported => "505".to_string(),
        }
    }
}
