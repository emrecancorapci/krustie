use dyn_clone::{clone_trait_object, DynClone};

use crate::{Request, Response};

/// Route handler trait
///
/// This trait is used to define the handler for the routes and middlewares.
pub trait RouteHandler: DynClone + Send {
    /// Handles the request and returns the result of the handler. It is used to define the handler for the routes and middlewares.
    fn handle(&mut self, request: &Request, response: &mut Response) -> HandlerResult;
}

#[derive(Debug, PartialEq, Eq)]
/// Result of the handler
///
/// `End` - Stops the execution of the handler chain
///
/// `Next` - Continues the execution of the handler chain
pub enum HandlerResult {
    /// **Stops** the execution of the handler chain. And the response is sent to the client.
    End,
    /// **Continues** the execution of the handler chain.
    Next,
}

clone_trait_object!(RouteHandler);
