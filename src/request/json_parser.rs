use super::HttpRequest;
use serde::de::{self, Error};
use serde_json::Result;

impl HttpRequest {
    pub fn body_to_json<'a, T>(&'a self) -> Result<T> where T: de::Deserialize<'a> {
        match self.body.as_ref() {
            None => {
                return Err(serde_json::Error::custom("No body"));
            }
            Some(body) => {
                let json: T = serde_json::from_slice(body.as_slice())?;
                Ok(json)
            }
        }
    }
}
