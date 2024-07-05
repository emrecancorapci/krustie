//! A router for handling requests
//!
//! It is used to handle requests and route them to the correct endpoint. Routers support sub-routers and middlewares.

use std::{ collections::HashMap, fmt::{ self, Debug, Formatter } };
use crate::{
    request::{ http_method::HttpMethod, HttpRequest },
    response::{ status_code::StatusCode, HttpResponse },
    server::route_handler::{ RouteHandler, HandlerResult },
    Middleware,
};

pub mod methods;

type Controller = fn(&HttpRequest, &mut HttpResponse);

/// A router for handling requests
///
/// # Example
///
/// ```rust
/// use krustie::{ Router, StatusCode };
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
    request_middlewares: Vec<Box<dyn Middleware>>,
    response_middlewares: Vec<Box<dyn Middleware>>,
}

impl Router {
    /// Creates a new router
    ///
    /// # Example
    ///
    /// To create a `GET` method for `/`
    ///
    /// ```rust
    /// use krustie::{ Router, StatusCode };
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
            request_middlewares: Vec::new(),
            response_middlewares: Vec::new(),
        }
    }

    /// Adds a router endpoint to the router
    ///
    /// # Example
    ///
    /// Create a 'POST' method for `/sub/suber`
    ///
    /// ```rust
    /// use krustie::{ Router, StatusCode };
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

    /// Adds a middleware to the router that will be executed before the request is handled
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Router, StatusCode, Middleware, HttpRequest, HttpResponse, server::route_handler::HandlerResult };
    ///
    /// struct AddHeader {
    ///   key: String,
    ///   value: String,
    /// }
    ///
    /// impl AddHeader {
    ///   fn new(key: &str, value: &str) -> Self {
    ///     Self { key: key.to_string(), value: value.to_string() }
    ///   }
    /// }
    ///
    /// impl Middleware for AddHeader {
    ///   fn middleware(&self, _: &HttpRequest, res: &mut HttpResponse) -> HandlerResult {
    ///     res.insert_header(&self.key, &self.value);
    ///     HandlerResult::Next
    ///   }
    /// }
    ///
    /// fn main() {
    ///   let mut router = Router::new();
    ///   let krustie_middleware = AddHeader::new("Server", "Krustie");
    ///
    ///   router.use_request_middleware(krustie_middleware);
    /// }
    /// ```
    pub fn use_request_middleware<T>(&mut self, middleware: T) where T: Middleware + 'static {
        self.request_middlewares.push(Box::new(middleware));
    }

    /// Adds a middleware to the router that will be executed before the request is handled
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Router, StatusCode, Middleware, HttpRequest, HttpResponse, server::route_handler::HandlerResult };
    ///
    /// struct AddHeader {
    ///   key: String,
    ///   value: String,
    /// }
    ///
    /// impl AddHeader {
    ///   fn new(key: &str, value: &str) -> Self {
    ///     Self { key: key.to_string(), value: value.to_string() }
    ///   }
    /// }
    ///
    /// impl Middleware for AddHeader {
    ///   fn middleware(&self, _: &HttpRequest, res: &mut HttpResponse) -> HandlerResult {
    ///     res.insert_header(&self.key, &self.value);
    ///     HandlerResult::Next
    ///   }
    /// }
    ///
    /// fn main() {
    ///   let mut router = Router::new();
    ///   let krustie_middleware = AddHeader::new("Server", "Krustie");
    ///
    ///   router.use_response_middleware(krustie_middleware);
    /// }
    /// ```
    pub fn use_response_middleware<T>(&mut self, middleware: T) where T: Middleware + 'static {
        self.response_middlewares.push(Box::new(middleware));
    }

    fn get_route(&self, path: &str) -> Result<&Router, &str> {
        for (key, router) in &self.subroutes {
            if key == path {
                return Ok(router);
            }
        }

        return Err("Route not found");
    }

    fn handle_router(
        &self,
        request: &HttpRequest,
        response: &mut HttpResponse,
        path: &Vec<String>
    ) -> HandlerResult {
        if path.len() == 0 {
            match self.endpoints.get(request.get_method()) {
                Some(endpoint) => {
                    endpoint(request, response);
                }
                None => {
                    response.status(StatusCode::MethodNotAllowed);
                    return HandlerResult::End;
                }
            }
        } else {
            match self.get_route(&path[0]) {
                Ok(router) => {
                    router.handle(request, response, &path[1..].to_vec());
                }
                Err(_) => {
                    response.status(StatusCode::NotFound);
                    return HandlerResult::End;
                }
            }
        }
        return HandlerResult::Next;
    }
}

impl Debug for Router {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Router {{ endpoints: {:?}, subroutes: {:?} }}", self.endpoints, self.subroutes)
    }
}

impl RouteHandler for Router {
    fn handle(
        &self,
        request: &HttpRequest,
        response: &mut HttpResponse,
        path: &Vec<String>
    ) -> HandlerResult {
        for middleware in &self.request_middlewares {
            match middleware.middleware(request, response) {
                HandlerResult::End => {
                    return HandlerResult::End;
                }
                HandlerResult::Next => (),
            }
        }

        match self.handle_router(request, response, path) {
            HandlerResult::End => {
                return HandlerResult::End;
            }
            HandlerResult::Next => (),
        }

        for middleware in &self.response_middlewares {
            match middleware.middleware(request, response) {
                HandlerResult::End => {
                    return HandlerResult::End;
                }
                HandlerResult::Next => (),
            }
        }

        return HandlerResult::Next;
    }
}
