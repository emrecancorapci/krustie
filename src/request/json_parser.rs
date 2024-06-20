use super::HttpRequest;
use serde::de::Error;
use serde_json::{ Result as JsonResult, Error as JsonError, from_slice };

pub use serde::Deserialize;

impl HttpRequest {
    pub fn json_from_struct<'a, T>(&'a self) -> JsonResult<T> where T: Deserialize<'a> {
        match self.body.as_ref() {
            None => {
                return Err(JsonError::custom("No body"));
            }
            Some(body) => {
                let json: T = from_slice(body.as_slice())?;
                Ok(json)
            }
        }
    }
}
