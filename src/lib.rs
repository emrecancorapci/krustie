#![warn(missing_debug_implementations, /* missing_docs,*/ rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]
//! # Krustie
//!
//! Krustie is a *simple* backend framework for Rust. It is inspired by *Express.js*
//! and is designed to be a simple and easy to use backend framework.

pub mod server;
pub mod router;
pub mod request;
pub mod response;
pub mod middleware;
pub mod json;


pub use server::Server;
pub use router::Router;
pub use request::HttpRequest;
pub use response::HttpResponse;
pub use middleware::Middleware;
pub use request::http_method::HttpMethod;
pub use response::status_code::StatusCode;
