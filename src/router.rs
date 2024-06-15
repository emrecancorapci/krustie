use std::collections::HashMap;

use crate::{
    request::{ HttpMethod, HttpRequest },
    response::HttpResponse,
    server::Handler,
};

pub mod methods;

type Controller = fn(&HttpRequest, &mut HttpResponse);

/// A router for handling requests
pub struct Router {
    base: String,
    endpoints: HashMap<HttpMethod, Controller>,
    subroutes: Vec<Router>,
}

impl Router {
    /// Creates a new router
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{router::Router, response::StatusCode};
    ///
    /// let mut router = Router::new("home");
    ///
    /// router.get(|req, res| {
    ///   res.status(StatusCode::Ok);
    /// });
    /// ```
    pub fn new(base: &str) -> Router {
        Router {
            base: base.to_string(),
            endpoints: HashMap::new(),
            subroutes: Vec::new(),
        }
    }

    /// Adds a router endpoint to the router
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::router::Router;
    ///
    /// let mut router = Router::new("home");
    /// let sub_router = Router::new("user");
    ///
    /// router.add_subrouter(sub_router);
    /// ```
    pub fn add_subrouter(&mut self, router: Router) {
        self.subroutes.push(router);
    }

    fn get_route(&self, path: &Vec<&str>) -> Result<&Router, &str> {
        let mut current_router = self;

        if current_router.base == path[0] {
            return Ok(current_router);
        }

        for route in path {
            match current_router.subroutes.iter().find(|r| r.base == *route) {
                Some(router) => {
                    current_router = router;
                }
                None => {
                    return Err("Route not found");
                }
            }
        }

        Ok(current_router)
    }

    fn get_endpoint(&self, method: HttpMethod) -> Option<&Controller> {
        self.endpoints.get(&method)
    }
}

impl Handler for Router {
    /// Handles routing of requests to the appropriate endpoint
    fn handle(&self, request: &HttpRequest, response: &mut HttpResponse) {
        let path = &request.request.path_array;

        match self.get_route(path) {
            Ok(router) => {
                match router.get_endpoint(request.request.method) {
                    Some(endpoint) => {
                        endpoint(request, response);
                    }
                    None => {}
                }
            }
            Err(_) => {}
        }
    }
}
