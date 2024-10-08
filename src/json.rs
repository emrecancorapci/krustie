//! This module provides utilities for working with JSON data.

pub use serde_json::json;
pub use serde_json::to_string as struct_to_string;
pub use serde_json::Result as JsonResult;
pub use serde_json::Value as JsonValue;

/// Converts a `JsonValue` (`serde_json::Value`) to a string
///
/// # Example
///
/// ```rust
/// use krustie::{ Request, Response, StatusCode, request::RequestBody, json::{ get_string_from_json, json } };
///
/// fn post_req(req: &Request, res: &mut Response) {
///   match req.get_body() {
///     RequestBody::Json(body) => {
///       let server_key_option = body.get("server");
///
///       match get_string_from_json(server_key_option).unwrap().as_str() {
///         "Krustie" => {
///           res.status(StatusCode::Ok).body_json(body.clone());
///         },
///         _ => {
///           res.status(StatusCode::try_from(201).unwrap()).body_json(
///             json!({"error": "Invalid server"})
///           );
///         }
///       }
///     },
///     _ => {
///         res.status(StatusCode::BadRequest).body_json(json!({"error": "Invalid JSON"}));
///     }
///   }
/// }
pub fn get_string_from_json(json_key: Option<&JsonValue>) -> Option<String> {
    json_key.map(|value| trim_json_string(value.to_string()))
}

fn trim_json_string(string: String) -> String {
    string.replace('"', "")
}
