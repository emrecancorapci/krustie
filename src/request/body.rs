use std::io::{Error, ErrorKind};

use crate::json::JsonValue;

// TODO: Add doctests
#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents the body of the HTTP request
///
/// The body can be of different types:
/// - Binary: Represents a binary body.
/// - Text: Represents a text body.
/// - Json: Represents a json body.
/// - None: Represents that there is no body or a body that is not supported.
pub enum RequestBody {
    /// Represents a binary body.
    Binary(Vec<u8>),
    /// Represents a text body.
    Text(String),
    /// Represents a json body.
    Json(JsonValue),
    // Represents a form body. Holds a HashMap of strings. **Not implemented yet.**
    // Form(HashMap<String, String>),
    /// Represents that there is no body or a body that is not supported.
    None,
}

impl RequestBody {
    pub(crate) fn parse(body: &[u8], content_type: &str) -> Result<RequestBody, Error> {
        if body.is_empty() {
            return Ok(RequestBody::None);
        }

        match content_type {
            "application/json" => match serde_json::from_slice(&body[..]) {
                Ok(json) => Ok(RequestBody::Json(json)),
                Err(_) => Err(Error::new(
                    ErrorKind::InvalidData,
                    "Error while parsing json body",
                )),
            },
            "plain/text" => Ok(RequestBody::Text(body.iter().map(|&c| c as char).collect())),
            _ => Ok(RequestBody::Binary(body.to_vec())),
        }
    }
}
