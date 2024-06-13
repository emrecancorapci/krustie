use super::StatusCode;

impl StatusCode {
    pub fn get_message(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::IAmATeapot => "I'm A Teapot",
            StatusCode::InternalServerError => "Internal Server Error",
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
            "400" => StatusCode::BadRequest,
            "404" => StatusCode::NotFound,
            "405" => StatusCode::MethodNotAllowed,
            "418" => StatusCode::IAmATeapot,
            "500" => StatusCode::InternalServerError,
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
            400 => StatusCode::BadRequest,
            404 => StatusCode::NotFound,
            405 => StatusCode::MethodNotAllowed,
            418 => StatusCode::IAmATeapot,
            500 => StatusCode::InternalServerError,
            _ => StatusCode::Ok,
        }
    }
}

impl From<StatusCode> for u16 {
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
    /// assert_eq!(u16::from(status_code_200), 200);
    /// assert_eq!(u16::from(status_code_418), 418);
    /// ```
    fn from(code: StatusCode) -> u16 {
        match code {
            StatusCode::Ok => 200,
            StatusCode::Created => 201,
            StatusCode::BadRequest => 400,
            StatusCode::NotFound => 404,
            StatusCode::MethodNotAllowed => 405,
            StatusCode::IAmATeapot => 418,
            StatusCode::InternalServerError => 500,
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
            StatusCode::BadRequest => "400".to_string(),
            StatusCode::NotFound => "404".to_string(),
            StatusCode::MethodNotAllowed => "405".to_string(),
            StatusCode::IAmATeapot => "418".to_string(),
            StatusCode::InternalServerError => "500".to_string(),
        }
    }
}