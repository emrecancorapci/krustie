use std::{collections::HashMap, io::Result};

use crate::http::core::{HttpResponse, HttpStatusCode};

use super::status_code::StatusCode;

#[derive(Debug)]
pub struct Response {
    version: String,
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    locals: HashMap<String, String>,
}

impl HttpResponse for Response {
    fn new() -> Self {
        Self {
            version: String::from("HTTP/1.1"),
            status: StatusCode::Ok,
            headers: HashMap::new(),
            body: Vec::new(),
            locals: HashMap::new(),
        }
    }
    fn set_status(&mut self, status_code: impl HttpStatusCode) -> &mut Self {
        self.status = StatusCode::try_from(status_code.code()).unwrap();
        self
    }

    fn set_header(&mut self, k: &str, v: &str) -> &mut Self {
        self.headers.insert(k.to_string(), v.to_string());
        self
    }

    fn set_body(&mut self, body: Vec<u8>, mime: &str) -> &mut Self {
        self.body = body;
        self.headers
            .insert(String::from("Content-Type"), mime.to_string());
        self
    }

    fn set_body_text(&mut self, text: &str) -> &mut Self {
        self.body = text.as_bytes().to_vec();
        self
    }

    fn send_file(&mut self, _path: &str) -> Result<&mut Self> {
        todo!()
    }

    fn extend_headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        self.headers.extend(headers);
        self
    }

    fn get_status(&self) -> &impl HttpStatusCode {
        &self.status
    }

    fn get_header(&self, k: &str) -> Option<&String> {
        self.headers.get(k)
    }

    fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    fn get_body(&self) -> &Vec<u8> {
        &self.body
    }

    fn get_header_mut(&mut self, k: &str) -> Option<&mut String> {
        self.headers.get_mut(k)
    }

    fn get_headers_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.headers
    }

    fn get_body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }

    fn set_cookie(&mut self, _key: &str, _value: &str) -> &mut Self {
        todo!()
    }

    fn get_cookie(&self, _key: &str) -> Option<&str> {
        todo!()
    }

    fn remove_cookie(&mut self, _key: &str) -> &mut Self {
        todo!()
    }

    fn redirect(&mut self, _url: &str, _status_code: Option<u16>) -> &mut Self {
        todo!()
    }

    fn get_local(&self, k: &str) -> Option<&String> {
        self.locals.get(k)
    }

    fn set_local(&mut self, k: &str, v: &str) -> Option<String> {
        self.locals.insert(k.to_string(), v.to_string())
    }
}

impl Into<Vec<u8>> for &mut Response {
    fn into(self) -> Vec<u8> {
        self.as_bytes()
    }
}

impl Into<Vec<u8>> for &Response {
    fn into(self) -> Vec<u8> {
        self.as_bytes()
    }
}

impl Into<Vec<u8>> for Response {
    fn into(self) -> Vec<u8> {
        self.as_bytes()
    }
}

impl Response {
    fn as_bytes(&self) -> Vec<u8> {
        let mut headers = self.headers.clone();
        let mut response = format!(
            "{http_version} {status_code} {status_msg}\r\n",
            http_version = self.version,
            status_code = self.status.code(),
            status_msg = self.status.msg()
        )
        .as_bytes()
        .to_vec();

        if !self.body.is_empty() {
            headers.insert("Content-Length".to_string(), self.body.len().to_string());

            if !headers.contains_key("Content-Type") {
                eprintln!("Content-Type not found even though body is present");

                headers.insert("Content-Type".to_string(), "text/plain".to_string());
            }
        } else {
            headers.insert("Content-Length".to_string(), "0".to_string());

            if !headers.contains_key("Content-Type") {
                headers.insert("Content-Type".to_string(), "text/plain".to_string());
            }
        }

        if !headers.is_empty() {
            let mut keys = headers.keys().collect::<Vec<&String>>();
            keys.sort();

            let headers_string = keys.iter().fold(String::new(), |acc, key| {
                format!("{acc}{key}: {value}\r\n", value = headers[*key])
            });

            response.extend_from_slice(headers_string.as_bytes());
        }

        response.extend_from_slice(b"\r\n");

        if !self.body.is_empty() {
            response.extend_from_slice(&self.body);
        }

        response
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.as_bytes()).unwrap())
    }
}
