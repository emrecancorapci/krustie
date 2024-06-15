use std::{ fs, io::Write, net::{ IpAddr, TcpListener, TcpStream }, path::PathBuf };

use crate::{ request::{ HttpMethod, HttpRequest }, response::{ HttpResponse, StatusCode } };

mod parser;

/// A server for handling requests
///
/// # Example
///
/// ```rust
/// use krustie::{ server::Server, router::Router, response::{ HttpResponse, StatusCode }, middleware::Middleware };
/// use std::collections::HashMap;
///
/// fn main() {
///     let mut server = Server::create(8080).unwrap();
///     let mut router = Router::new("home");
///
///     router
///         .get(|_, res| {
///             res.status(StatusCode::Ok);
///         })
///         .post(|_, res| {
///             res.status(StatusCode::Ok);
///         });
///
///     let middleware = Middleware::new(|_, res: &mut HttpResponse| {
///         let mut headers: HashMap<String, String> = HashMap::new();
///         headers.insert(String::from("Server"), String::from("Rust"));
///         res.headers(headers);
///     });
///
///     server.use_handler(router);
///     server.use_handler(middleware);
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
    ///
    /// let server = Server::create(8080).unwrap();
    ///
    /// // server.listen();
    /// ```
    pub fn create(port: u16) -> Result<Server, String> {
        let addr = format!("127.0.0.1:{port}");

        match TcpListener::bind(addr) {
            Ok(listener) => {
                Ok(Server {
                    request_handlers: Vec::new(),
                    listener,
                    is_serves_static: false,
                    static_path: String::from("./public"),
                    // listener_ip: None,
                })
            }
            Err(err) => { Err(err.to_string()) }
        }
    }

    /// Serves static files from the specified path
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::server::Server;
    ///
    /// let mut server = Server::create(8080).unwrap();
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

    fn handle_stream(&self, stream: &mut TcpStream) {
        let parsed = Server::parse_stream(&stream);
        let mut response = HttpResponse::default();

        match parsed {
            Ok((headers, body)) => {
                self.handle_request(headers, body, &mut response);
            }
            Err(error) => {
                response.status(StatusCode::BadRequest).debug_msg(&error);
            }
        }

        match stream.write_all(&response.as_bytes()[..]) {
            Ok(_) => {}
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    fn handle_request(&self, headers: Vec<String>, body: String, response: &mut HttpResponse) {
        let request = HttpRequest::new(&headers, &body);
        if self.is_serves_static && &request.request.method == &HttpMethod::GET {
            match
                Server::serve_static_files(
                    &request.request.path_array[0],
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
        self.request_handlers.iter().for_each(|handler| handler.handle(&request, response));
    }

    /// Adds a middleware or router to the server
    /// ```rust
    /// use krustie::{ server::Server, router::Router, response::{ HttpResponse, StatusCode }, middleware::Middleware };
    /// use std::collections::HashMap;
    ///
    /// let mut server = Server::create(8080).unwrap();
    /// let mut router = Router::new("home");
    ///
    /// let middleware = Middleware::new(|_, res: &mut HttpResponse| {
    ///     let mut headers: HashMap<String, String> = HashMap::new();
    ///     headers.insert(String::from("Server"), String::from("Rust"));
    ///     res.headers(headers);
    /// });
    ///
    /// server.use_handler(router);
    /// server.use_handler(middleware);
    /// ```
    pub fn use_handler<F>(&mut self, handler: F) where F: Handler + 'static {
        self.request_handlers.push(Box::new(handler));
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
