//! # Endpoint
//!
//! Endpoint is a struct that holds the information about a route in the router. It holds the
//! information about the method, controller and middlewares that should be applied to the route.

use crate::{HttpMethod, Middleware};

use super::Controller;

/// Endpoint struct that holds the information about a route in the router
///
/// # Example
///
/// ```rust
/// use krustie::{ Server, Router, Endpoint, HttpMethod, StatusCode, Request, Response };
///
/// fn get(req: &Request, res: &mut Response) {
///   res.status(StatusCode::Ok).body_text("Hello, World!");
/// }
///
/// let mut server = Server::create();
/// let mut router = Router::new();
/// let endpoint = Endpoint::new(HttpMethod::GET, get);
///
/// router.use_endpoint("/hello", endpoint);
///
/// server.use_handler(router);
///
/// // server.listen(8080);
/// ```
#[derive(Debug)]
pub struct Endpoint {
    method: HttpMethod,
    controller: Controller,
    middlewares: Vec<Box<dyn Middleware>>,
}

impl Endpoint {
    /// Creates a new Endpoint instance
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Endpoint, HttpMethod, Request, Response, StatusCode };
    ///
    /// fn get(req: &Request, res: &mut Response) {
    ///   res.status(StatusCode::Ok).body_text("Hello, World!");
    /// }
    ///
    /// let endpoint = Endpoint::new(HttpMethod::GET, get);
    /// ```
    pub fn new(method: HttpMethod, controller: Controller) -> Self {
        Self {
            method,
            controller,
            middlewares: Vec::new(),
        }
    }

    fn new_with_middleware(
        method: HttpMethod,
        controller: Controller,
        middlewares: Vec<Box<dyn Middleware>>,
    ) -> Self {
        Self {
            method,
            controller,
            middlewares,
        }
    }

    pub(crate) fn is_method(&self, method: &HttpMethod) -> bool {
        self.method == *method
    }

    pub(crate) fn get_controller(&self) -> &Controller {
        &self.controller
    }

    pub(crate) fn get_middlewares(&mut self) -> &mut Vec<Box<dyn Middleware>> {
        &mut self.middlewares
    }
}

impl Clone for Endpoint {
    fn clone(&self) -> Self {
        Self {
            method: self.method.clone(),
            controller: self.controller.clone(),
            middlewares: self.middlewares.clone(),
        }
    }
}
