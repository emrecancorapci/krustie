use crate::{Request, Response, StatusCode};
use std::{
    fmt::{Debug, Formatter},
    io::Write,
    net::{TcpListener, TcpStream},
};

pub mod route_handler;
pub mod testing;
use route_handler::{HandlerResult, RouteHandler};
use std::thread;

#[doc = include_str!("../docs/core/server.md")]
pub struct Server {
    route_handlers: Vec<Box<dyn RouteHandler + Send>>,
    address: String,
}

impl Server {
    /// Creates a new server instance
    ///
    /// # Example
    ///
    /// ```no_run
    /// use krustie::Server;
    ///
    /// let server = Server::create();
    ///
    /// server.listen(8080);
    /// ```
    pub fn create() -> Self {
        Self {
            route_handlers: Vec::new(),
            address: String::from(""),
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
    /// use krustie::{ Server, Router, Response, StatusCode, Middleware, middleware::gzip::GzipEncoder };
    ///
    /// let mut server = Server::create();
    /// let mut router = Router::new();
    ///
    /// server.use_handler(router);
    /// server.use_handler(GzipEncoder);
    /// ```
    pub fn use_handler(&mut self, handler: impl RouteHandler + 'static) {
        self.route_handlers.push(Box::new(handler));
    }

    /// Listens for incoming requests on the specified IP and port
    ///
    /// # Example
    ///
    /// ```no_run
    /// use krustie::Server;
    ///
    /// let mut server = Server::create();
    ///
    /// server.listen(8080);
    /// ```
    pub fn listen(self, port: u16) {
        Listener::listen(port, self);
    }

    /// Handles data stream comes from TcpListener
    /// 
    /// Use this if you to use your own listener for the server
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use krustie::Server;
    /// use std::net::TcpListener;
    ///
    /// let mut server = Server::create();
    /// let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    /// 
    /// for stream in listener.incoming() {
    ///   server.handle_stream(&mut stream.unwrap());
    /// }
    /// 
    /// ```
    pub fn handle_stream(&mut self, stream: &mut TcpStream) {
        let mut response = Response::default();

        match Request::parse(stream) {
            Ok(request) => {
                for handler in &mut self.route_handlers {
                    let result = handler.handle(&request, &mut response);
                    if result == HandlerResult::End {
                        break;
                    }
                }
            }
            Err(err) => {
                response
                    .status(StatusCode::BadRequest)
                    .debug_msg(&err.to_string());
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

impl Clone for Server {
    fn clone(&self) -> Self {
        let route_handlers: Vec<Box<dyn RouteHandler + Send>> = self.route_handlers.clone();
        Self {
            route_handlers,
            address: self.address.clone(),
        }
    }
}

impl Debug for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Server {{ Address: {} }}", self.address)
    }
}

#[derive(Debug)]
struct Listener {}

impl Listener {
    fn listen(port: u16, handler: Server) {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address).unwrap_or_else(|err| panic!("{}", err));

        println!("Listening on http://localhost:{port}");

        for stream_result in listener.incoming() {
            let mut stream = stream_result.unwrap_or_else(|err| {
                panic!("Error while listening: {}", err);
            });

            let mut handler = handler.clone();

            thread::spawn(move || {
                handler.handle_stream(&mut stream);
            });
        }
    }
}
