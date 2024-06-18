use super::HttpRequest;
use serde::de::Error;
use serde_json::{ Result as JsonResult, Error as JsonError, from_slice as json_from_slice };

pub use serde::de::Deserialize;

impl HttpRequest {
    pub fn body_as_json<'a, T>(&'a self) -> JsonResult<T> where T: Deserialize<'a> {
        match self.body.as_ref() {
            None => {
                return Err(JsonError::custom("No body"));
            }
            Some(body) => {
                let json: T = json_from_slice(body.as_slice())?;
                Ok(json)
            }
        }
    }
}
