use super::http_method::HttpMethod;

#[derive(Eq, Hash, PartialEq)]
pub struct Route {
    path: String,
    method: HttpMethod,
}

impl Route {
    pub fn new(path: &str, method: HttpMethod) -> Route {
        Route {
            path: path.to_string(),
            method,
        }
    }
}
