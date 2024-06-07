use std::fmt::{ Display, Error, Formatter };

#[derive(Eq, Hash, PartialEq)]
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
    pub fn new(method: &str) -> Result<HttpMethod, &str> {
        let binding = method.to_ascii_lowercase();

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

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::DELETE => write!(f, "DELETE"),
        }
    }
}
