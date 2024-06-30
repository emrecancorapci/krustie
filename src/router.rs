use std::collections::HashMap;
use crate::{
    middleware::MiddlewareHandler,
    request::{ http_method::HttpMethod, HttpRequest },
    response::{ HttpResponse, status_code::StatusCode },
    server::Handler,
};

pub mod methods;

type Controller = fn(&HttpRequest, &mut HttpResponse);

/// A router for handling requests
///
/// # Example
///
/// ```rust
/// use krustie::{ router::{ Router, methods::Endpoints }, response::StatusCode };
///
/// let mut main_router = Router::new();
/// let mut sub_router = Router::new();
/// let mut sub_sub_router = Router::new();
///
/// sub_sub_router
///   .get(|req, res| {
///     res.status(StatusCode::Ok);
///   })
///   .post(|req, res| {
///     res.status(StatusCode::Ok);
///   });
///
/// sub_router.use_router("suber", sub_sub_router);
/// main_router.use_router("sub", sub_router);
/// ```
pub struct Router {
    endpoints: HashMap<HttpMethod, Controller>,
    subroutes: HashMap<String, Router>,
    request_middleware: Vec<Box<dyn MiddlewareHandler>>,
    response_middleware: Vec<Box<dyn MiddlewareHandler>>,
}

impl Router {
    /// Creates a new router
    ///
    /// # Example
    ///
    /// To create a `GET` method for `/`
    ///
    /// ```rust
    /// use krustie::{ router::{ Router, methods::Endpoints }, response::StatusCode };
    ///
    /// let mut main_router = Router::new();
    ///
    /// main_router.get(|req, res| {
    ///   res.status(StatusCode::Ok);
    /// });
    ///
    /// ```
    pub fn new() -> Self {
        Self {
            endpoints: HashMap::new(),
            subroutes: HashMap::new(),
            request_middleware: Vec::new(),
            response_middleware: Vec::new(),
        }
    }

    /// Adds a router endpoint to the router
    ///
    /// # Example
    ///
    /// Create a 'POST' method for `/sub/suber`
    ///
    /// ```rust
    /// use krustie::{ router::{ Router, methods::Endpoints }, response::StatusCode };
    ///
    /// let mut main_router = Router::new();
    /// let mut sub_router = Router::new();
    /// let mut sub_sub_router = Router::new();
    ///
    /// sub_sub_router.post(|req, res| {
    ///   res.status(StatusCode::Ok);
    /// });
    ///
    /// sub_router.use_router("suber", sub_sub_router);
    /// main_router.use_router("sub", sub_router);
    /// ```
    pub fn use_router(&mut self, path: &str, router: Router) {
        let path = if path.starts_with("/") { &path[1..] } else { path };

        self.subroutes.entry(path.to_string()).or_insert(router);
    }

    pub fn add_request_middleware<T>(&mut self, middleware: T) where T: MiddlewareHandler + 'static {
        self.request_middleware.push(Box::new(middleware));
    }

    pub fn add_response_middleware<T>(&mut self, middleware: T) where T: MiddlewareHandler + 'static {
        self.response_middleware.push(Box::new(middleware));
    }

    fn handle_route(&self, request: &HttpRequest, response: &mut HttpResponse, path: &Vec<String>) {
        for middleware in &self.request_middleware {
            middleware.handle(request, response);
        }

        if path.len() == 1 {
            if let Some(endpoint) = self.endpoints.get(request.get_method()) {
                endpoint(request, response);
            }
        } else {
            if let Ok(router) = self.get_route(&path[1]) {
                router.handle_route(request, response, &path[1..].to_vec());
            } else {
                response.status(StatusCode::NotFound);
            }
        }

        for middleware in &self.response_middleware {
            middleware.handle(request, response);
        }
    }

    fn get_route(&self, path: &str) -> Result<&Router, &str> {
        for (key, router) in &self.subroutes {
            if key == path {
                return Ok(router);
            }
        }

        return Err("Route not found");
    }
}

impl Handler for Router {
    /// Handles routing of requests to the appropriate endpoint
    fn handle(&self, request: &HttpRequest, response: &mut HttpResponse) {
        let path = &request.get_path_array();

        if path.len() > 0 {
            for (key, router) in &self.subroutes {
                if key == &path[0] {
                    router.handle_route(request, response, path);
                    return;
                }
            }
        }
    }
}
