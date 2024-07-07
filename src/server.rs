//! Server module for handling requests
//!
//! # A Complete Example
//!
//! ```rust
//! use krustie::{
//!     Server,
//!     Router,
//!     HttpRequest,
//!     HttpResponse,
//!     Middleware,
//!     StatusCode,
//!     json::{ get_string_from_json, json },
//!     middleware::{ GzipEncoder, ServeStaticFiles },
//!     server::route_handler::HandlerResult,
//!     request::BodyType,
//! };
//!
//! struct AddHeader {
//!     key: String,
//!     value: String,
//! }
//!
//! impl AddHeader {
//!     fn new(key: &str, value: &str) -> Self {
//!         Self { key: key.to_string(), value: value.to_string() }
//!     }
//! }
//!
//! impl Middleware for AddHeader {
//!     fn middleware(&self, _: &HttpRequest, res: &mut HttpResponse) -> HandlerResult {
//!         res.insert_header(&self.key, &self.value);
//!         HandlerResult::Next
//!     }
//! }
//!
//! fn main() {
//!     let mut server = Server::create();
//!     let krustie_middleware = AddHeader::new("Server", "Krustie");
//!     let mut router = Router::new();
//!
//!     router.get(|_, res| {
//!         res.status(StatusCode::Ok).body(
//!             b"<html><body><h1>Hello, World!</h1></body></html>".to_vec(),
//!             "text/html"
//!         );
//!     });
//!
//!     let mut sub_router = Router::new();
//!
//!     sub_router
//!         .get(|_, res| {
//!             let body = json!({"message": "Hello, World!"});
//!             res.status(StatusCode::Ok).json_body(body);
//!         })
//!         .post(post_req);
//!
//!     router.use_router("/home", sub_router);
//!
//!     server.use_handler(router);
//!     server.use_handler(krustie_middleware);
//!     server.use_handler(GzipEncoder);
//!     server.use_handler(ServeStaticFiles::new("public"));
//!     
//!     // vvvvvv Uncommment to listen on
//!     // server.listen((127, 0, 0, 1), 8080);
//! }
//!
//! fn post_req(req: &HttpRequest, res: &mut HttpResponse) {
//!   match req.get_body() {
//!     BodyType::Json(json) => {
//!       let key_result = json.get("server");
//!       if get_string_from_json(key_result).unwrap() == "Krustie" {
//!         res.status(StatusCode::Ok).json_body(json!({"message": "Valid server"}));
//!       } else {
//!         res.status(StatusCode::try_from(201).unwrap())
//!           .json_body(json!({"error": "Invalid server"}));
//!       }
//!     },
//!     _ => {
//!       res.status(StatusCode::BadRequest).json_body(json!({"error": "Invalid JSON"}));
//!     }
//!   }
//! }
//! ```

use std::{ fmt::{ Debug, Formatter }, io::Write, net::{ TcpListener, TcpStream } };

use route_handler::{ HandlerResult, RouteHandler };

use crate::{
    request:: HttpRequest,
    response::{ status_code::StatusCode, HttpResponse },
};

pub mod route_handler;

/// A server for handling requests
///
/// # Example
///
/// ```rust
/// use krustie::{ Server, Router, StatusCode };
///
/// fn main() {
///     let mut server = Server::create();
///     let mut router = Router::new();
///
///     router.get(|_, res| {
///         res.status(StatusCode::Ok)
///             .body(b"Hello World!".to_vec(), "text/plain");
///     });
///
///     server.use_handler(router);
///
///     // server.listen((127, 0, 0, 1), 8080);
/// }
/// ```
pub struct Server {
    route_handlers: Vec<Box<dyn RouteHandler>>,
    address: String,
    // listener_ip: Option<IpAddr>,
}

impl Server {
    /// Creates a new server instance
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::Server;
    ///
    /// let server = Server::create();
    ///
    /// // vvvvvv Uncommment to listen on
    /// // server.listen((127, 0, 0, 1), 8080)
    /// ```
    pub fn create() -> Self {
        Self {
            route_handlers: Vec::new(),
            address: String::from(""),
        }
    }

    /// Listens for incoming requests on the specified IP and port
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::Server;
    ///
    /// let mut server = Server::create();
    ///
    /// // vvvvvv Uncommment to listen on
    /// // server.listen((127, 0, 0, 1), 8080);
    /// ```
    pub fn listen(&mut self, ip: (u8, u8, u8, u8), port: u16) {
        self.address = format!("{}.{}.{}.{}:{}", ip.0, ip.1, ip.2, ip.3, port);
        match TcpListener::bind(&self.address) {
            Ok(listener) => {
                for stream_result in listener.incoming() {
                    match stream_result {
                        Ok(mut stream) => {
                            self.handle_stream(&mut stream);
                        }
                        Err(e) => {
                            eprintln!("Error while listening: {}", e);
                        }
                    }
                }
            }
            Err(err) => { panic!("{}", err) }
        }
    }

    /// Adds a middleware or a router to the server
    ///
    /// `Middleware` are functions that are executed before or after the request is handled by the server.
    /// They can be used to modify the request or response.
    ///
    /// `Router` is a collection of routes that can be used to handle requests.
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Server, Router, HttpResponse, StatusCode, Middleware, middleware::gzip::GzipEncoder };
    ///
    /// let mut server = Server::create();
    /// let mut router = Router::new();
    ///
    ///
    /// server.use_handler(router);
    /// server.use_handler(GzipEncoder);
    /// ```
    pub fn use_handler(&mut self, handler: impl RouteHandler + 'static) {
        self.route_handlers.push(Box::new(handler));
    }

    fn handle_stream(&self, stream: &mut TcpStream) {
        let mut response = HttpResponse::default();

        match HttpRequest::parse(&stream) {
            Ok(request) => {
                for handler in &self.route_handlers {
                    match handler.handle(&request, &mut response, &request.get_path_array()) {
                        HandlerResult::End => {
                            break;
                        }
                        HandlerResult::Next => {
                            continue;
                        }
                    }
                }
            }
            Err(error) => {
                response.status(StatusCode::BadRequest).debug_msg(&error);
            }
        }

        let response_stream: Vec<u8> = response.into();

        match stream.write_all(&response_stream) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}

impl Debug for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Server {{ Address: {} }}", self.address)
    }
}
