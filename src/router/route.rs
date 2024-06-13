use crate::request::HttpMethod;

#[derive(Eq, Hash, PartialEq)]
pub struct Route {
    path: String,
    method: HttpMethod,
}

impl Route {
    pub fn new(path: &str, method: &HttpMethod) -> Route {
        let method = method.clone();
        Route {
            path: path.to_string(),
            method,
        }
    }
}
