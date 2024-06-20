pub use serde_json::json;
pub use serde_json::Value as JsonValue;
pub use serde_json::Result as JsonResult;
pub use serde_json::to_string as struct_to_string;

pub fn get_string_from_json(json_key: Option<&JsonValue>) -> Option<String> {
    json_key.map(|value| trim_json_string(value.to_string()))
}

pub fn trim_json_string(string: String) -> String {
    string.replace("\"", "")
}