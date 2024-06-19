use std::{ fs, io::Write, net::{ Ipv4Addr, TcpListener, TcpStream }, path::PathBuf };

use crate::{
    request::{ parser::Parse, HttpMethod, HttpRequest },
    response::{ HttpResponse, StatusCode },
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
///   middleware::{ MiddlewareHandler, Middleware, gzip::Gzip } };
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
///   let mut server = Server::create(Ipv4Addr::new(127, 0, 0, 1), 8080);
///   let mut router = Router::new();
///   let mut sub_router = Router::new();
///
///   sub_router
///     .get(|_, res| {
///       res.status(StatusCode::Ok);
///     })
///     .post(|_, res| {
///       res.status(StatusCode::try_from(201).unwrap());
///     });
///
///   router.use_router("home", sub_router);
///
///   server.use_handler(router);
///   server.use_handler(AddKrustieHeader::get_middleware());
///   server.use_handler(Gzip::get_middleware());
/// }
/// ```
pub struct Server {
    request_handlers: Vec<Box<dyn Handler>>,
    listener: TcpListener,
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
    /// let server = Server::create(Ipv4Addr::new(127, 0, 0, 1), 8080);
    ///
    /// // server.listen();
    /// ```
    pub fn create(ip: Ipv4Addr, port: u16) -> Self {
        let addr = format!("{ip}:{port}");

        match TcpListener::bind(addr) {
            Ok(listener) => {
                Self {
                    request_handlers: Vec::new(),
                    listener,
                    is_serves_static: false,
                    static_path: String::from("./public"),
                    // listener_ip: None,
                }
            }
            Err(err) => { panic!("{}", err) }
        }
    }

    pub fn create_local(port: u16) -> Self {
        Self::create(Ipv4Addr::new(127, 0, 0, 1), port)
    }

    /// Serves static files from the specified path
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::server::Server;
    ///
    /// let mut server = Server::create_local(8080);
    ///
    /// server.serve_static("./public");
    /// ```
    pub fn serve_static(&mut self, path: &str) {
        self.is_serves_static = true;
        self.static_path = path.to_string();
    }

    pub fn listen(&mut self) {
        // match self.listener.accept() {
        //     Ok((_, addr)) => {
        //         self.listener_ip = Some(addr.ip());
        //     }
        //     Err(e) => {
        //         println!("error: {}", e);
        //         return;
        //     }
        // }
        for stream_result in self.listener.incoming() {
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

    /// Adds a middleware or router to the server
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use krustie::{ server::Server, router::Router, response::{ HttpResponse, StatusCode }, middleware::{ Middleware, gzip::Gzip } };
    /// use std::collections::HashMap;
    ///
    /// let mut server = Server::create_local(8080);
    /// let mut router = Router::new();
    ///
    ///
    /// server.use_handler(router);
    /// server.use_handler(Gzip::get_middleware());
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

pub trait Handler {
    fn handle(&self, request: &HttpRequest, response: &mut HttpResponse);
}
