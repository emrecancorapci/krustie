use std::{ io::Write, net::TcpListener };

use crate::{ request::HttpRequest, response::{ HttpResponse, StatusCode }, router::Router };

pub mod parser;
pub struct Server {
    router: Router,
    listener: TcpListener,
    is_serves_static: bool,
    static_path: String,
}

impl Server {
    pub fn create(port: u16) -> Result<Server, String> {
        let addr = format!("127.0.0.1:{port}");

        match TcpListener::bind(addr) {
            Ok(listener) => {
                Ok(Server {
                    router: Router::new(),
                    listener,
                })
            }
            Err(err) => { Err(err.to_string()) }
        }
    }

    /// Serves static files from the specified path
    pub fn serve_static(&mut self, path: &str) {
        self.is_serves_static = true;
        self.static_path = path.to_string();
    }

    pub fn listen(&self) {
        for stream_result in self.listener.incoming() {
            match stream_result {
                Ok(mut stream) => {
                    let parsing = Server::parse_stream(&stream);
                    let mut response = HttpResponse::new();

                    match parsing {
                        Ok((headers, body)) => {
                            let request = &HttpRequest::from(&headers, &body);

                            if self.is_serves_static && request.request.method == "GET" {
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

                            self.router.handle(request, &mut response);
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

    pub fn use_router(&mut self, router: Router) {
        self.router = router;
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
