use std::{collections::HashMap, net::IpAddr};

use self::request_line::RequestLine;

pub mod http_method;
pub mod methods;

mod request_line;

pub struct HttpRequest<'a> {
    pub request: RequestLine<'a>,
    pub headers: HashMap<String, String>,
    pub body: &'a str,
    pub ip: IpAddr,
}


#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    // CONNTECT,
    // HEAD,
    // OPTIONS,
    // TRACE,
}

impl HttpMethod {
    /// Create a new HttpMethod from a string.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use krustie::request::HttpMethod;
    /// 
    /// let method = HttpMethod::new("GET").expect("Method not found");
    /// 
    /// assert_eq!(method, HttpMethod::GET);
    /// ```
    pub fn new(method: &str) -> Result<HttpMethod, &str> {
        let binding = method.to_lowercase();

        match binding.as_str() {
            "get" => {
                return Ok(HttpMethod::GET);
            }
            "post" => {
                return Ok(HttpMethod::POST);
            }
            "put" => {
                return Ok(HttpMethod::PUT);
            }
            "patch" => {
                return Ok(HttpMethod::PATCH);
            }
            "delete" => {
                return Ok(HttpMethod::DELETE);
            }
            &_ => { Err("Method not found.") }
        }
    }
}