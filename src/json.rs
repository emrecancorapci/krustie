//! This module provides utilities for working with JSON data.

pub use serde_json::json;
pub use serde_json::Value as JsonValue;
pub use serde_json::Result as JsonResult;
pub use serde_json::to_string as struct_to_string;

/// Converts a `JsonValue` (`serde_json::Value`) to a string
///
/// # Example
///
/// ```rust
/// fn post_req(req: &Request, res: &mut Response) {
///   match req.get_body() {
///     RequestBody::Json(body) => {
///       let server_key_option = body.get("server");
///
///       match get_string_from_json(server_key_option).unwrap() {
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
