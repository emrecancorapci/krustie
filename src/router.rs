use std::collections::HashMap;

use crate::{ request::HttpRequest, response::{ HttpResponse, StatusCode } };

type IController = Box<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync>;

pub struct Router {
    endpoints: HashMap<Route, IController>,
}

impl Router {
    /// Creates a new router
    pub fn new() -> Router {
        Router {
            endpoints: HashMap::new(),
        }
    }

    /// Adds a GET endpoint to the router
    pub fn get(&mut self, path: &str, controller: IController) {
        self.endpoints.insert(
            Route {
                path: path.to_string(),
                method: HttpMethod::GET,
            },
            controller
        );
    }

    /// Adds a POST endpoint to the router
    pub fn post(&mut self, path: &str, controller: IController) {
        self.endpoints.insert(
            Route {
                path: path.to_string(),
                method: HttpMethod::POST,
            },
            controller
        );
    }

    /// Handles routing of requests
    pub fn handle(&self, request: &HttpRequest, response: &mut HttpResponse) {
        let path = &request.request.path_array;

        match HttpMethod::from(&request.request.method) {
            Ok(method) => {
                let route = Route::new(path[0], method);

                match self.endpoints.get(&route) {
                    Some(controller) => controller(request, response),
                    None => {
                        response.status(StatusCode::NotFound);
                    }
                }
            }
            Err(_) => {
                let body = b"Incorrect method type".to_vec();
                response.status(StatusCode::BadRequest).body(body, "plain/text");
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Route {
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

#[derive(Eq, Hash, PartialEq)]
enum HttpMethod {
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
    pub fn from(method_str: &str) -> Result<HttpMethod, &str> {
        let binding = method_str.to_ascii_lowercase();
        let method = binding.as_str();

        match method {
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
