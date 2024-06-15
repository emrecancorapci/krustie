use std::{
    collections::HashMap,
    fs,
    io::Write,
    net::{ IpAddr, TcpListener, TcpStream },
    path::PathBuf,
};

use self::request_handler::{ RequestHandler, Middleware };

use crate::{
    request::{ HttpMethod, HttpRequest },
    response::{ HttpResponse, StatusCode },
    router::Router,
};

mod request_handler;
mod parser;

pub struct Server {
    request_handlers: Vec<RequestHandler>,
    listener: TcpListener,
    is_serves_static: bool,
    static_path: String,
    listener_ip: Option<IpAddr>,
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
                    listener_ip: None,
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
        match self.listener.accept() {
            Ok((_, addr)) => {
                self.listener_ip = Some(addr.ip());
            }
            Err(e) => {
                println!("error: {}", e);
                return;
            }
        }
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
        self.request_handlers
            .iter()
            .for_each(|handler| handler.run(&request, response));
    }

    /// Parses the incoming stream
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{server::Server, router::Router, response::StatusCode};
    ///
    /// let mut server = Server::create(8080).unwrap();
    /// let mut router = Router::new();
    ///
    /// router.get("/", Box::new(|req, res| {
    ///   res.status(StatusCode::Ok);
    /// }));
    ///
    /// server.use_router(router);
    /// ```
    pub fn use_router(&mut self, router: Router) {
        self.request_handlers.push(RequestHandler::Router(router));
    }

    /// Parses the incoming stream
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{server::Server, router::Router, response::{HttpResponse, StatusCode}};
    /// use std::collections::HashMap;
    ///
    /// let mut server = Server::create(8080).unwrap();
    ///
    /// server.use_middleware(Box::new(|_, res: &mut HttpResponse| {
    ///     let mut headers: HashMap<String, String> = HashMap::new();
    ///     headers.insert(String::from("Server"), String::from("Rust"));
    ///     res.status(StatusCode::Ok).headers(headers);
    ///   })
    /// );
    /// ```
    pub fn use_middleware(&mut self, middleware: Middleware) {
        self.request_handlers.push(RequestHandler::Middleware(middleware));
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

pub enum DataObject {
    Array(Vec<DataObject>),
    Object(HashMap<String, DataObject>),
    String(String),
    Int(isize),
    Float(f64),
    Boolean(bool),
    Null,
}
