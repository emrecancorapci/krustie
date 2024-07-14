use std::collections::HashMap;

use crate::Response;

impl Response {
    /// Adds a single header to the response
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///    response.insert_header("Server", "Krustie");
    /// }
    /// ```
    pub fn insert_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Gets the headers of the response
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request };
    ///
    /// fn get(request: &Request, response: Response) {
    ///   let headers = response.get_headers();
    ///
    ///   for (key, value) in headers.iter() {
    ///    println!("{}: {}", key, value);
    ///   }
    /// }
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Gets requested header from the response
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request };
    ///
    /// fn get(request: &Request, response: Response) {
    ///   let server_header = response.get_header("Server");
    ///
    ///   match server_header {
    ///     Some(header) => println!("Server header: {}", header),
    ///     None => println!("Server header not found")
    ///   }
    /// }
    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    /// Gets the body of the response as a byte vector reference
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request, json::json };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///  let body = response.get_body();
    /// }
    pub fn get_body(&mut self) -> &Vec<u8> {
        &mut self.body
    }

    /// Gets the body of the response as a **mutable** byte vector reference
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request, json::json };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///  let body = response.get_body();
    /// }
    pub fn get_body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }

    /// Updates the body of the response.
    ///
    /// Function sets `Content-Length` automatically but needs `Content-Type` to be set manually.
    ///
    /// # Errors
    ///
    /// Returns an error if the request has no body already.
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Response, StatusCode, Request, json::json };
    ///
    /// fn get(request: &Request, response: &mut Response) {
    ///   response.body(b"Hello, World!".to_vec(), "text/plain");
    ///
    ///   response.update_body(b"Goodbye, Mars!".to_vec());
    /// }
    pub fn update_body(&mut self, body: Vec<u8>) -> Result<(), String> {
        if self.body.len() == 0 {
            return Err("Request has no body.".to_string());
        }
        self.headers.insert(String::from("Content-Length"), body.len().to_string());
        self.body = body;
        return Ok(());
    }
}
