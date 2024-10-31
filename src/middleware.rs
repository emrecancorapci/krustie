use dyn_clone::DynClone;
use std::fmt::Debug;

use crate::{HandlerResult, Request, Response, RouteHandler};

pub mod gzip;
pub mod rate_limiter;
pub mod statics;

pub use self::{gzip::GzipEncoder, rate_limiter::RateLimiter, statics::ServeStatic};

#[doc = include_str!("../docs/core/middleware.md")]
pub trait Middleware: DynClone + Send {
    /// This function has to be implemented for the created [Middleware]. Function will return a [HandlerResult] to determine if the middleware should continue the execution or stop it.
    ///
    /// For the middleware to be executed and continue the execution, it should return [HandlerResult::Next].
    ///
    /// If the middleware should stop the execution (e.g. return 404), it should return [HandlerResult::End].
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
