use crate::{HttpMethod, Middleware};

use super::Controller;

#[doc = include_str!("../../docs/core/endpoint.md")]
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

    /// Creates a new Endpoint instance with middleware
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use krustie::{ Endpoint, HttpMethod, Request, Response, Router, StatusCode, Middleware, HandlerResult };
    /// 
    /// let mut router = Router::new();
    /// 
    /// fn get(req: &Request, res: &mut Response) {
    ///   res.status(StatusCode::Ok).body_text("Hello, World!");
    /// }
    /// 
    /// #[derive(Clone)]
    /// struct MyMiddleware;
    /// 
    /// impl Middleware for MyMiddleware {
    ///   fn middleware(&mut self, req: &Request, res: &mut Response) -> HandlerResult {
    ///     HandlerResult::Next
    ///   }
    /// }
    /// 
    /// let endpoint = Endpoint::new_with_middleware(HttpMethod::GET, get, vec![Box::new(MyMiddleware)]);
    /// 
    /// router.use_endpoint("/", endpoint);
    /// ```
    pub fn new_with_middleware(
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
