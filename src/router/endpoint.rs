use crate::{ HttpMethod, Middleware };

use super::Controller;

#[derive(Debug)]
pub struct Endpoint {
    method: HttpMethod,
    pub(crate) controller: Controller,
    middlewares: Vec<Box<dyn Middleware>>,
}

impl Endpoint {
    pub fn new(method: HttpMethod, controller: Controller) -> Self {
        Self {
            method,
            controller,
            middlewares: Vec::new(),
        }
    }

    pub fn new_with_middleware(
        method: HttpMethod,
        controller: Controller,
        middlewares: Vec<Box<dyn Middleware>>
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
}
