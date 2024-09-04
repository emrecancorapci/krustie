//! Middleware module for Krustie.
//!
//! Middleware is a function that can be executed before or after the request is handled.
//!
//! To create a middleware, implement the `Middleware` trait. Which has a single function called `middleware`.
//!
//! It takes itself as `&self`, a `Request` and a `Response` as arguments and returns a `HandlerResult`.

use dyn_clone::DynClone;
use std::fmt::Debug;

use crate::{
    server::route_handler::{HandlerResult, RouteHandler},
    Request, Response,
};

pub mod gzip;
pub mod rate_limiter;
pub mod statics;
pub mod helmet;

pub use self::{gzip::GzipEncoder, rate_limiter::RateLimiter, statics::ServeStatic};

/// Middleware trait to be implemented for creating middleware.
///
/// If there is no property declared in the struct, struct can be used directly.
/// Or it can be used as a value if it needs to be initialized.
///
/// # Example
///
/// - In this example `AddKrustieHeader` can be used as `server.add_handler(AddKrustieHeader)`
///
/// ```rust
/// use krustie::{ Request, Response, Middleware, server::route_handler::HandlerResult };
///
/// #[derive(Clone)]
/// struct AddKrustieHeader;
///
/// impl AddKrustieHeader {
///   fn add_header(res: &mut Response) {
///     res.insert_header("Server", "Krustie");
///   }
/// }
///
/// impl Middleware for AddKrustieHeader {
///   fn middleware(&mut self, req: &Request, res: &mut Response) -> HandlerResult {
///     AddKrustieHeader::add_header(res);
///     HandlerResult::Next
///   }
/// }
/// ```
///
/// - In this example `AddHeader` should be initialized.
///
/// ```rust
/// use krustie::{ Request, Response, Middleware, server::route_handler::HandlerResult };
///
/// #[derive(Clone)]
/// struct AddHeader {
///     server: String,
///     value: String,
/// }
///
/// impl AddHeader {
///     fn new(server: &str, value: &str) -> Self {
///         Self { server: server.to_string(), value: value.to_string() }
///     }
/// }
///
/// impl Middleware for AddHeader {
///   fn middleware(&mut self, _: &Request, res: &mut Response) -> HandlerResult {
///     res.insert_header(&self.server, &self.value);
///     HandlerResult::Next
///   }
/// }
/// ```
///
pub trait Middleware: DynClone + Send {
    /// Middleware function to be implemented for the middleware.
    ///
    /// For the middleware to be executed and continue the execution, it should return `HandlerResult::Next`.
    ///
    /// If the middleware should stop the execution (e.g. return 404), it should return `HandlerResult::Stop`.
    fn middleware(&mut self, request: &Request, response: &mut Response) -> HandlerResult;
}

impl<T> RouteHandler for T
where
    T: Middleware,
{
    fn handle(&mut self, request: &Request, response: &mut Response) -> HandlerResult {
        T::middleware(self, request, response)
    }
}

impl Debug for dyn Middleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Middleware",)
    }
}

dyn_clone::clone_trait_object!(Middleware);
