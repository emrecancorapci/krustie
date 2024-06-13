use std::collections::HashMap;

use crate::{ request::HttpRequest, response::{ HttpResponse, StatusCode } };

use self::http_method::HttpMethod;
use self::route::Route;

type Controller = Box<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync>;

pub mod http_method;
pub mod route;

pub struct Router {
    endpoints: HashMap<Route, Endpoint>,
}

impl Router {
    /// Creates a new router
    pub fn new() -> Router {
        Router {
            endpoints: HashMap::new(),
        }
    }

    pub fn add_router(&mut self, path: &str, router: Router) {
        self.endpoints.insert(Route::new(path, HttpMethod::GET), Endpoint::Router(router));
    }

    /// Adds a GET endpoint to the router
    pub fn get(&mut self, path: &str, controller: Controller) {
        self.endpoints.insert(Route::new(path, HttpMethod::GET), Endpoint::Controller(controller));
    }

    /// Adds a POST endpoint to the router
    pub fn post(&mut self, path: &str, controller: Controller) {
        self.endpoints.insert(Route::new(path, HttpMethod::POST), Endpoint::Controller(controller));
    }

    /// Adds a PUT endpoint to the router
    pub fn put(&mut self, path: &str, controller: Controller) {
        self.endpoints.insert(Route::new(path, HttpMethod::PUT), Endpoint::Controller(controller));
    }

    /// Adds a DELETE endpoint to the router
    pub fn delete(&mut self, path: &str, controller: Controller) {
        self.endpoints.insert(Route::new(path, HttpMethod::DELETE), Endpoint::Controller(controller));
    }

    /// Adds a PATCH endpoint to the router
    pub fn patch(&mut self, path: &str, controller: Controller) {
        self.endpoints.insert(Route::new(path, HttpMethod::PATCH), Endpoint::Controller(controller));
    }

    /// Handles routing of requests to the appropriate endpoint
    pub fn handle(&self, request: &HttpRequest, response: &mut HttpResponse) {
        let path = &request.request.path_array;

        let route = Route::new(path[0], &request.request.method);

        match self.endpoints.get(&route) {
            Some(endpoint) => {
                endpoint.run(request, response);
            }
            None => {
                response.status(StatusCode::NotFound);
                return;
            }
        }
    }
}

enum Endpoint {
    Controller(Controller),
    Router(Router),
}

impl Endpoint {
    /// Runs the endpoint depending on the type
    pub fn run(&self, req: &HttpRequest, res: &mut HttpResponse) {
        match self {
            Endpoint::Controller(controller) => {
                controller(req, res);
            }
            Endpoint::Router(router) => {
                router.handle(req, res);
            }
        }
    }
}