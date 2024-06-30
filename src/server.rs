use std::{ fmt::{ Debug, Formatter }, io::Write, net::{ TcpListener, TcpStream } };

use route_handler::{ HandlerResult, RouteHandler };

use crate::{
    request::{ request_parser::Parse, HttpRequest },
    response::{ status_code::StatusCode, HttpResponse },
};

pub mod route_handler;

/// A server for handling requests
///
/// # Example
///
/// ```rust
/// use krustie::{
///   Server, Router, HttpResponse, HttpRequest, StatusCode, Middleware,
///   router::methods::Endpoints,
///   middleware::Gzip,
///   json::{ json, get_string_from_json },
/// };
/// use std::collections::HashMap;
/// use std::net::Ipv4Addr;
///
/// struct AddKrustieHeader;
///
/// impl Middleware for AddKrustieHeader {
///   fn middleware(req: &HttpRequest, res: &mut HttpResponse) {
///     res.insert_header("Server", "Krustie");
///   }
/// }
///
/// fn main() {
///   let mut server = Server::create();
///   let mut router = Router::new();
///   let mut sub_router = Router::new();
///
///   sub_router
///     .get(|_, res| {
///       let body = json!({"message": "Hello, World!"});
///       res.status(StatusCode::Ok).json_body(body);
///     })
///     .post(post_req);
///
///   router.use_router("home", sub_router);
///
///   server.use_handler(router);
///   server.use_handler(AddKrustieHeader);
///   server.use_handler(Gzip);
///
///   // vvvvvv Uncommment to listen on
///   // server.listen((127, 0, 0, 1), 8080);
/// }
///
/// fn post_req(req: &HttpRequest, res: &mut HttpResponse) {
///   match req.get_body_as_json() {
///     Ok(body) => {
///       if get_string_from_json(body.get("server")).unwrap() == "Krustie" {
///         res.status(StatusCode::Ok).json_body(body);
///       } else {
///         res.status(StatusCode::try_from(201).unwrap()).json_body(json!({"error": "Invalid server"}));
///       }
///     }
///     Err(_) => {
///       res.status(StatusCode::BadRequest).json_body(json!({"error": "Invalid JSON"}));
///     }
///   }
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
                            println!("error: {}", e);
                        }
                    }
                }
            }
            Err(err) => { panic!("{}", err) }
        }
    }

    /// Adds a middleware or router to the server
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Server, Router, HttpResponse, StatusCode, Middleware, middleware::gzip::Gzip };
    /// use std::collections::HashMap;
    ///
    /// let mut server = Server::create();
    /// let mut router = Router::new();
    ///
    ///
    /// server.use_handler(router);
    /// server.use_handler(Gzip);
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
                println!("error: {}", e);
            }
        }
    }
}

impl Debug for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Server {{ Address: {} }}", self.address)
    }
}
