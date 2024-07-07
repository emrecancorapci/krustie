//! This module contains the BodyType enum and its implementation.

use std::collections::HashMap;

use crate::json::JsonValue;

#[derive(Debug)]
/// Represents the body of the HTTP request
/// 
/// It can be a text, a json, a form or none.
pub enum BodyType {
    /// Represents a text body. Holds a vector of bytes.
    Text(Vec<u8>),
    /// Represents a json body. Holds a JsonValue.
    Json(JsonValue),
    /// Represents a form body. Holds a HashMap of strings. **Not implemented yet.**
    Form(HashMap<String, String>),
    /// Represents that there is no body or a body that is not supported.
    None,
}

impl BodyType {
    pub(crate) fn parse(body: Vec<u8>, content_type: &str) -> Result<BodyType, String> {
        let body = match content_type {
            "application/json" => {
                match serde_json::from_slice(&body[..]) {
                    Ok(json) => BodyType::Json(json),
                    Err(_) => BodyType::None,
                }
            }
            "plain/text" => { BodyType::Text(body.to_vec()) }
            _ => {
                let error = format!(
                    "Error while parsing body. Content-type not supported: {}",
                    content_type
                );
                return Err(error);
            }
        };
        Ok(body)
    }
}