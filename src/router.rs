use std::collections::HashMap;

use crate::{ request::HttpRequest, response::{ HttpResponse, StatusCode } };

use http_method::HttpMethod;
use route::Route;

type IController = Box<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync>;

pub mod http_method;
pub mod route;

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

    /// Serves static files from the specified path
    pub fn serve_static(&mut self, path: &str) {
        self.is_serves_static = true;
        self.static_path = path.to_string();
    }

    /// Adds a GET endpoint to the router
    pub fn get(&mut self, path: &str, controller: IController) {
        self.endpoints.insert(Route::new(path, HttpMethod::GET), controller);
    }

    /// Adds a POST endpoint to the router
    pub fn post(&mut self, path: &str, controller: IController) {
        self.endpoints.insert(Route::new(path, HttpMethod::POST), controller);
    }

    /// Handles routing of requests
    pub fn handle(&self, request: &HttpRequest, response: &mut HttpResponse) {
        let path = &request.request.path_array;

        match HttpMethod::new(&request.request.method) {
            Ok(method) => {
                let route = Route::new(path[0], method);

                match self.endpoints.get(&route) {
                    Some(controller) => {
                        return controller(request, response);
                    }
                    None => {
                        response.status(StatusCode::NotFound);
                        return;
                    }
                }
            }
            Err(_) => {
                let body = b"Incorrect method type".to_vec();
                response.status(StatusCode::BadRequest).body(body, "plain/text");
                return;
            }
        }
    }
}
