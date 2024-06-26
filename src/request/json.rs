use std::io::{ Error, ErrorKind };
use serde_json::{ from_slice, Value };

impl super::HttpRequest {
    pub fn get_body_as_json(&self) -> Result<Value, Error> {
        match self.body.as_ref() {
            None => {
                return Err(Error::new(ErrorKind::InvalidData, "No body found"));
            }
            Some(body) => {
                match from_slice(body.as_slice()) {
                    Ok(json) => Ok(json),
                    Err(e) => {
                        return Err(Error::new(ErrorKind::InvalidData, e.to_string()));
                    }
                }
            }
        }
    }
}
