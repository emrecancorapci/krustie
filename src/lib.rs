#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    clippy::all
)]
#![allow(clippy::needless_return)]
#![forbid(unsafe_code)]

//! # Krustie
//!
//! Krustie is a *simple* backend framework that inspired by *Express.js* and is designed to be a
//! simple and easy to use backend framework.
//!
//! It is capable of handling requests, routing them to the correct endpoint, modifying the request
//! and response using middlewares,serving static files, and encoding responses using Gzip.
//!
//! Biggest flaws it has right now is that it is *not async* and it is *not optimized* for
//! performance. So it is **not recommended** to use it in production.

pub mod server;
// pub mod old_router;
pub mod request;
pub mod response;
pub mod middleware;
pub mod json;
pub mod router;

pub use server::Server;
pub use router::Router;
pub use request::Request;
pub use response::Response;
pub use middleware::Middleware;
pub use request::http_method::HttpMethod;
pub use response::status_code::StatusCode;
pub use router::endpoint::Endpoint;
