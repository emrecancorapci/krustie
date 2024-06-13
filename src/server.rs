use std::{ fs, io::Write, net::{ IpAddr, TcpListener }, path::PathBuf };

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

    pub fn listen(&self) {
        let ip: IpAddr;
        match self.listener.accept() {
            Ok((_, addr)) => {
                ip = addr.ip();
            }
            Err(e) => {
                println!("error: {}", e);
                return;
            }
        }
        for stream_result in self.listener.incoming() {
            match stream_result {
                Ok(mut stream) => {
                    let parsed = Server::parse_stream(&stream);
                    let mut response = HttpResponse::default();

                    match parsed {
                        Ok((headers, body)) => {
                            let request = HttpRequest::new(&headers, &body, ip);

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
                                .for_each(|handler| handler.run(&request, &mut response));
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
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
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
