use std::{ collections::HashMap, fs, path::PathBuf };
use http_method::HttpMethod;
use route::Route;

use super::{ request::HttpRequest, response::{ HttpResponse, StatusCode } };

type IController = Box<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync>;

pub mod http_method;
pub mod route;

pub struct Router {
    endpoints: HashMap<Route, IController>,
    is_serves_static: bool,
    static_path: String,
}

impl Router {
    /// Creates a new router
    pub fn new() -> Router {
        Router {
            endpoints: HashMap::new(),
            is_serves_static: false,
            static_path: String::from("./public"),
        }
    }

    /// Serves static files from the specified path
    pub fn serve_static(&mut self, path: &str) {
        self.is_serves_static = true;
        self.static_path = path.to_string();
    }

    /// Adds a GET endpoint to the router
    pub fn get(&mut self, path: &str, controller: IController) {
        self.endpoints.insert(Route::new(path, HttpMethod::GET), controller);
    }

    /// Adds a POST endpoint to the router
    pub fn post(&mut self, path: &str, controller: IController) {
        self.endpoints.insert(Route::new(path, HttpMethod::POST), controller);
    }

    /// Handles routing of requests
    pub fn handle(&self, request: &HttpRequest, response: &mut HttpResponse) {
        let path = &request.request.path_array;

        match HttpMethod::new(&request.request.method) {
            Ok(method) => {
                if self.is_serves_static && &method == &HttpMethod::GET {
                    match Router::serve_static_files(&path[0], self.static_path.as_str()) {
                        Some(content) => {
                            response.status(StatusCode::Ok).body(content, "html/text");
                            return;
                        }
                        None => {}
                    }
                }

                let route = Route::new(path[0], method);

                match self.endpoints.get(&route) {
                    Some(controller) => {
                        return controller(request, response);
                    }
                    None => {
                        response.status(StatusCode::NotFound);
                        return;
                    }
                }
            }
            Err(_) => {
                let body = b"Incorrect method type".to_vec();
                response.status(StatusCode::BadRequest).body(body, "plain/text");
                return;
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
