//! A router for handling requests
//!
//! It is used to handle requests and route them to the correct endpoint. Routers support sub-routers and middlewares.
//!
//! # Example
//!
//! ```rust
//! use krustie::{ Router, StatusCode };
//!
//! let mut main_router = Router::new();
//! let mut sub_router = Router::new();
//! let mut sub_sub_router = Router::new();
//!
//! sub_sub_router
//!   .get(|req, res| {
//!     res.status(StatusCode::Ok);
//!   })
//!   .post(|req, res| {
//!     res.status(StatusCode::Ok);
//!   });
//!
//! sub_router.use_router("suber", sub_sub_router);
//! main_router.use_router("sub", sub_router);
//! ```

use crate::{
    server::route_handler::{ HandlerResult, RouteHandler },
    HttpMethod,
    Request,
    Response,
    Middleware,
    StatusCode,
};
use std::{ collections::HashMap, fmt::{ Debug, Formatter, Result as fmtResult } };

pub mod methods;

type Controller = fn(&Request, &mut Response);

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
        let sub_path = if let Some(path) = path.strip_prefix('/') { &path[1..] } else { path };

        self.subroutes.entry(sub_path.to_string()).or_insert(router);
    }

    /// Adds a middleware to the router that will be executed before the request is handled
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Router, StatusCode, Middleware, Request, Response, server::route_handler::HandlerResult };
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
    ///   fn middleware(&mut self, _: &Request, res: &mut Response) -> HandlerResult {
    ///     res.insert_header(&self.key, &self.value);
    ///     HandlerResult::Next
    ///   }
    /// }
    ///
    /// let mut router = Router::new();
    /// let krustie_middleware = AddHeader::new("Server", "Krustie");
    ///
    /// router.use_request_middleware(krustie_middleware);
    /// ```
    pub fn use_request_middleware<T>(&mut self, middleware: T) where T: Middleware + 'static {
        self.request_middlewares.push(Box::new(middleware));
    }

    /// Adds a middleware to the router that will be executed before the request is handled
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Router, StatusCode, Middleware, Request, Response, server::route_handler::HandlerResult };
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
    ///   fn middleware(&mut self, _: &Request, res: &mut Response) -> HandlerResult {
    ///     res.insert_header(&self.key, &self.value);
    ///     HandlerResult::Next
    ///   }
    /// }
    ///
    /// let mut router = Router::new();
    /// let krustie_middleware = AddHeader::new("Server", "Krustie");
    ///
    /// router.use_response_middleware(krustie_middleware);
    /// ```
    pub fn use_response_middleware<T>(&mut self, middleware: T) where T: Middleware + 'static {
        self.response_middlewares.push(Box::new(middleware));
    }

    fn handle_router(
        &mut self,
        request: &Request,
        response: &mut Response,
        path: &[String]
    ) -> HandlerResult {
        if path.is_empty() || path[0].is_empty() {
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
            match self.subroutes.get_mut(&path[0]) {
                Some(router) => {
                    router.handle(request, response, &path[1..]);
                }
                None => {
                    response.status(StatusCode::NotFound);
                    return HandlerResult::End;
                }
            }
        }
        return HandlerResult::Next;
    }
}

impl Debug for Router {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "Router {{ endpoints: {:?}, subroutes: {:?} }}", self.endpoints, self.subroutes)
    }
}

impl RouteHandler for Router {
    fn handle(
        &mut self,
        request: &Request,
        response: &mut Response,
        path: &[String]
    ) -> HandlerResult {
        for middleware in &mut self.request_middlewares {
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

        for middleware in &mut self.response_middlewares {
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

impl Default for Router {
    /// Creates a new router
    ///
    /// # Example
    ///
    /// To create a `GET` method for `/`
    ///
    /// ```rust
    /// use krustie::{ Router, StatusCode };
    ///
    /// let mut main_router = Router::default();
    ///
    /// main_router.get(|req, res| {
    ///   res.status(StatusCode::Ok);
    /// });
    ///
    /// ```
    fn default() -> Self {
        return Self::new();
    }
}
