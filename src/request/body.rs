//! This module contains the RequestBody enum and its implementation.
//!
//! RequestBody can be a `Text`, a `Json` or `None`.
//!
//! - Text returns a `Vec<u8>`
//!
//! - Json returns a `JsonValue` (it's json_verde::Value)

use std::io::{ Error, ErrorKind };

use crate::json::JsonValue;

#[derive(Debug, Clone)]
/// Represents the body of the HTTP request
pub enum RequestBody {
    /// Represents a text body. Holds a vector of bytes.
    Text(Vec<u8>),
    /// Represents a json body. Holds a JsonValue.
    Json(JsonValue),
    // Represents a form body. Holds a HashMap of strings. **Not implemented yet.**
    // Form(HashMap<String, String>),
    /// Represents that there is no body or a body that is not supported.
    None,
}

impl RequestBody {
    pub(crate) fn parse(body: Vec<u8>, content_type: &str) -> Result<RequestBody, Error> {
        let body = match content_type {
            "application/json" => {
                match serde_json::from_slice(&body[..]) {
                    Ok(json) => RequestBody::Json(json),
                    Err(_) => RequestBody::None,
                }
            }
            "plain/text" => { RequestBody::Text(body.to_vec()) }
            _ => {
                let error =
                    format!("Error while parsing body. Content-type not supported: {}", content_type);
                return Err(Error::new(ErrorKind::InvalidInput, error));
            }
        };
        Ok(body)
    }
}
