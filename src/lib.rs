#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2024_compatibility,
    rust_2024_guarded_string_incompatible_syntax,
    unreachable_pub,
    clippy::all,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    rustdoc::unescaped_backticks
)]
#![allow(clippy::needless_return)]
#![forbid(unsafe_code)]
#![doc(issue_tracker_base_url = "https://github.com/emrecancorapci/krustie/issues")]

//! # Krustie
//!
//! Krustie is a *very simple* backend framework that inspired by *Express.js* and is designed to be a
//!  easy to use.
//!
//! It is capable of handling requests, routing them to the correct endpoint, modifying the request
//! and response using middlewares, serving static files, and encoding responses using Gzip.
//!
//! Biggest flaws it has right now is that it is *not async* and it is *not optimized* for
//! performance.
//!
//! <div class="warning">
//!
//! Krustie is still in the early stages of development and is not yet ready for production use. The API is subject to change and there may be bugs or missing features.
//!
//! </div>
//!
//! ## Hello World Example
//!
//! Here is an example of how to create a basic web server using Krustie:
//!
//! ```no_run
//! use krustie::{Router, Server, StatusCode};
//!
//! fn main() {
//!     let mut server = Server::create();
//!     let mut main_router = Router::new();
//!
//!     main_router.get("/", |_, res| {
//!         res.status(StatusCode::Ok).body_text("Hello World!");
//!     });
//!
//!     server.use_handler(main_router);
//!
//!     server.listen(8080);
//! }
//! ```
//!
//! The app starts a server on port `8080` and responds with `Hello World!` to any incoming *GET* request to the root path (`/`).
//!
//! The [Router] object is used to define the routes and the [Server] object is used to start the server and listen for incoming requests.
//!
//! ## Running the Server
//!
//! To run the server, use the following command:
//!
//! ```sh
//! cargo run
//! ```
//!
//! When the server is running, you can access it by opening a web browser and navigating to `http://localhost:8080`.
//!
//! ## Building the Server
//!
//! To build the server, use the following command:
//!
//! ```sh
//! cargo build
//! ```
//!
//! This will create an executable file in the `target/debug` directory.
//!

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
/// Contains built-in middlewares that can be used with Krustie.
pub mod middlewares {
    pub use crate::middleware::GzipEncoder;
    pub use crate::middleware::RateLimiter;
    pub use crate::middleware::ServeStatic;
}

#[doc(inline)]
pub use middleware::Middleware;
#[doc(inline)]
pub use request::builder::RequestBuilder;
#[doc(inline)]
pub use request::http_method::HttpMethod;
#[doc(inline)]
pub use request::Request;
#[doc(inline)]
pub use request::RequestBody;
#[doc(inline)]
pub use response::status_code::StatusCode;
#[doc(inline)]
pub use response::Response;
#[doc(inline)]
pub use router::endpoint::Endpoint;
#[doc(inline)]
pub use router::Router;
#[doc(inline)]
pub use server::route_handler::HandlerResult;
#[doc(inline)]
pub use server::route_handler::RouteHandler;
#[doc(inline)]
pub use server::Server;
