use std::{ io::Write, net::TcpListener };

use crate::{ request::HttpRequest, response::{ HttpResponse, StatusCode }, router::Router };

pub struct Server {
    router: Router,
    listener: TcpListener,
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

    pub fn listen(&self) {
        for stream_result in self.listener.incoming() {
            match stream_result {
                Ok(mut stream) => {
                    let parsing = Server::parse_stream(&stream);
                    let mut response = HttpResponse::new();

                    match parsing {
                        Ok((headers, body)) => {
                            let request = &HttpRequest::from(&headers, &body);
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
}
