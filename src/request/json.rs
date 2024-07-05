//! JSON utilities for the HttpRequest struct

use std::io::{ Error, ErrorKind };
use serde_json::{ from_slice, Value };

impl super::HttpRequest {
    /// Get the body of the request as a JSON object
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ HttpResponse, StatusCode, HttpRequest, json::{ json, get_string_from_json } };
    ///
    ///fn post_req(req: &HttpRequest, res: &mut HttpResponse) {
    ///    match req.get_body_as_json() {
    ///        Ok(body) => {
    ///            let server_key_option = body.get("server");
    ///            if get_string_from_json(server_key_option).unwrap() == "Krustie" {
    ///                res.status(StatusCode::Ok).json_body(body);
    ///            } else {
    ///                res.status(StatusCode::try_from(201).unwrap()).json_body(
    ///                    json!({"error": "Invalid server"})
    ///                );
    ///            }
    ///        }
    ///        Err(_) => {
    ///            res.status(StatusCode::BadRequest).json_body(json!({"error": "Invalid JSON"}));
    ///        }
    ///    }
    ///}
    ///   `````
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
