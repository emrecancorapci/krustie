//! This module contains the RequestBody enum and its implementation.
//!
//! RequestBody can be a `Text`, a `Json` or `None`.
//!
//! - Text returns a `Vec<u8>`
//!
//! - Json returns a `JsonValue` (it's json_verde::Value)

use std::io::{Error, ErrorKind};

use crate::json::JsonValue;

#[derive(Debug, Clone)]
// TODO: Add doctests
/// Represents the body of the HTTP request
pub enum RequestBody {
    /// Represents a binary body. Holds a vector of bytes.
    Binary(Vec<u8>),
    /// Represents a text body. Holds a vector of bytes.
    Text(String),
    /// Represents a json body. Holds a JsonValue.
    Json(JsonValue),
    // Represents a form body. Holds a HashMap of strings. **Not implemented yet.**
    // Form(HashMap<String, String>),
    /// Represents that there is no body or a body that is not supported.
    None,
}

impl RequestBody {
    pub(crate) fn parse(body: Vec<u8>, content_type: &str) -> Result<RequestBody, Error> {
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
            _ => Ok(RequestBody::Binary(body)),
        }
    }
}
