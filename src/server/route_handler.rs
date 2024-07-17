//! Route handler module
//!
//! This module contains the `RouteHandler` trait and `HandlerResult` enum.
//!
//! The `RouteHandler` trait is used to define the handler for the routes and middlewares.
//!
//! The `HandlerResult` enum is used to define the result of the handler.

use crate::{ Request, Response };

/// Route handler trait
///
/// This trait is used to define the handler for the routes and middlewares.
pub trait RouteHandler {
    /// Handles the request and returns the result of the handler. It is used to define the handler for the routes and middlewares.
    fn handle(&mut self, request: &Request, response: &mut Response, path: &[String]) -> HandlerResult;
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
