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

pub mod json;
#[doc(hidden)]
pub mod middleware;
#[doc(hidden)]
pub mod request;
#[doc(hidden)]
pub mod response;
#[doc(hidden)]
pub mod router;
#[doc(hidden)]
pub mod server;

#[doc(inline)]
pub use middleware::Middleware;
#[doc(inline)]
pub use request::http_method::HttpMethod;
#[doc(inline)]
pub use request::Request;
#[doc(inline)]
pub use response::status_code::StatusCode;
#[doc(inline)]
pub use response::Response;
#[doc(inline)]
pub use router::endpoint::Endpoint;
#[doc(inline)]
pub use router::Router;
#[doc(inline)]
pub use server::Server;
