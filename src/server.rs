use std::{
    fmt::{ Debug, Formatter },
    fs,
    io::Write,
    net::{ TcpListener, TcpStream },
    path::PathBuf,
};

use crate::{
    request::{ request_parser::Parse, http_method::HttpMethod, HttpRequest },
    response::{ HttpResponse, status_code::StatusCode },
};

/// A server for handling requests
///
/// # Example
///
/// ```rust
/// use krustie::{
///   server::Server,
///   router::{ Router, methods::Endpoints },
///   response::{ HttpResponse, StatusCode },
///   request::HttpRequest,
///   middleware::{ MiddlewareHandler, Middleware, gzip::Gzip },
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
    request_handlers: Vec<Box<dyn Handler>>,
    address: String,
    // listener_ip: Option<IpAddr>,
    // Static file serving
    static_path: String,
    is_serves_static: bool,
}

impl Server {
    /// Creates a new server instance
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::server::Server;
    /// use std::net::Ipv4Addr;
    ///
    /// let server = Server::create();
    ///
    /// // vvvvvv Uncommment to listen on
    /// // server.listen((127, 0, 0, 1), 8080)
    /// ```
    pub fn create() -> Self {
        Self {
            address: String::from(""),
            request_handlers: Vec::new(),
            is_serves_static: false,
            static_path: String::from("./public"),
        }
    }

    /// Serves static files from the specified path
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::server::Server;
    ///
    /// let mut server = Server::create();
    ///
    /// server.serve_static("./public");
    /// ```
    pub fn serve_static(&mut self, path: &str) {
        self.is_serves_static = true;
        self.static_path = path.to_string();
    }

    /// Listens for incoming requests on the specified IP and port
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::server::Server;
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
    /// use krustie::{ server::Server, router::Router, response::{ HttpResponse, StatusCode }, middleware::{ Middleware, gzip::Gzip } };
    /// use std::collections::HashMap;
    ///
    /// let mut server = Server::create();
    /// let mut router = Router::new();
    ///
    ///
    /// server.use_handler(router);
    /// server.use_handler(Gzip);
    /// ```
    pub fn use_handler(&mut self, handler: impl Handler + 'static) {
        self.request_handlers.push(Box::new(handler));
    }

    fn handle_stream(&self, stream: &mut TcpStream) {
        let mut response = HttpResponse::default();

        match HttpRequest::parse(&stream) {
            Ok(request) => {
                if self.is_serves_static && request.get_method() == &HttpMethod::GET {
                    match
                        Self::serve_static_files(
                            &request.get_path_array()[0],
                            self.static_path.as_str()
                        )
                    {
                        Some(content) => {
                            response.status(StatusCode::Ok).body(content, "html/text");
                            return;
                        }
                        None => {}
                    }
                }
                self.request_handlers
                    .iter()
                    .for_each(|handler| handler.handle(&request, &mut response));
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

    fn serve_static_files(file_name: &str, folder_path: &str) -> Option<Vec<u8>> {
        let path = PathBuf::from(folder_path).join(file_name);

        match fs::read(path) {
            Ok(content) => {
                return Some(content);
            }
            Err(_) => {
                return None;
            }
        }
    }
}

impl Debug for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Server {{ Address: {}, \r\nStatic: {}, Static Path: {}}}",
            self.address,
            if self.is_serves_static {
                "Enabled"
            } else {
                "Disabled"
            },
            self.static_path
        )
    }
}

pub trait Handler {
    fn handle(&self, request: &HttpRequest, response: &mut HttpResponse);
}
