use std::{ collections::HashMap, fmt::Display };
use crate::{
    server::route_handler::{ HandlerResult, RouteHandler },
    HttpMethod,
    Middleware,
    Request,
    Response,
    StatusCode,
};
use regex::Regex;

pub mod methods;

type Controller = fn(&Request, &mut Response);
type RouterResult<'a> = Option<(&'a Endpoint, HashMap<String, String>)>;

// TODO: Look at Radix Tree

#[derive(Debug)]
pub struct Router {
    endpoints: Vec<Endpoint>,
    middlewares: Vec<Box<dyn Middleware>>,
    subdirs: HashMap<String, Box<Router>>,
    param_dir: Option<(String, Box<Router>)>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            endpoints: Vec::new(),
            middlewares: Vec::new(),
            subdirs: HashMap::new(),
            param_dir: None,
        }
    }

    pub fn use_router(&mut self, path: &str, router: Router) {
        let path = path.trim();

        if path == "/" || path.is_empty() {
            panic!("Route already exist.");
        }

        let path_types = Self::get_path_types(path);
        let mut types_iter = path_types.into_iter().peekable();

        Self::add_router(self, router, &mut types_iter);
    }

    pub fn use_endpoint(&mut self, path: &str, endpoint: Endpoint) {
        let path = path.trim();

        if path == "/" || path.is_empty() {
            self.endpoints.push(endpoint);
        } else {
            let path_types = Self::get_path_types(path);
            let mut types_iter = path_types.into_iter().peekable();

            Self::add_endpoint(self, endpoint, &mut types_iter)
        }
    }

    pub(crate) fn route_handler<'a>(
        &'a self,
        path_array: &Vec<String>,
        method: &HttpMethod
    ) -> RouterResult<'a> {
        let params: HashMap<String, String> = HashMap::new();
        let iter: std::slice::Iter<'_, String> = path_array.iter();

        return Self::handle_routes(self, method, params, iter);
    }

    fn add_router<'a>(
        router: &'a mut Router,
        new_router: Router,
        iter: &mut std::iter::Peekable<std::vec::IntoIter<PathType>>
    ) {
        if iter.next().is_none() {
            panic!("Route already exist.");
        }

        match iter.next().unwrap() {
            PathType::Subdirectory(path) => {
                if let Some(found_router) = router.subdirs.get_mut(&path) {
                    // Router Found
                    Self::add_router(found_router, new_router, iter);
                } else if iter.peek().is_some() {
                    // No Router & Iteration Continues
                    let mut inserted_router = Box::new(Router::new());
                    Self::add_router(inserted_router.as_mut(), new_router, iter);
                    router.subdirs.insert(path, inserted_router);
                } else {
                    // No Router & Iteration Ends
                    router.subdirs.insert(path, Box::new(new_router));
                }
            }
            PathType::Parameter(param) => {
                if let Some(found_router) = &mut router.param_dir {
                    // Router Found
                    Self::add_router(found_router.1.as_mut(), new_router, iter);
                } else if iter.peek().is_some() {
                    // No Router & Iteration Continues
                    let mut inserted_router = Box::new(Router::new());
                    Self::add_router(inserted_router.as_mut(), new_router, iter);
                    router.param_dir = Some((param, Box::new(Router::new())));
                } else {
                    // No Router & Iteration Ends
                    router.param_dir = Some((param, Box::new(new_router)));
                }
            }
        }
    }

    fn add_endpoint<'a>(
        router: &'a mut Router,
        endpoint: Endpoint,
        iter: &mut std::iter::Peekable<std::vec::IntoIter<PathType>>
    ) {
        match iter.next() {
            Some(PathType::Subdirectory(path)) => {
                if let Some(found_router) = router.subdirs.get_mut(&path) {
                    // Router Found
                    if iter.peek().is_some() {
                        Self::add_endpoint(found_router, endpoint, iter);
                    } else {
                        router.endpoints.push(endpoint);
                    }
                } else if iter.peek().is_some() {
                    // No Router & Iteration Continues
                    router.subdirs.insert(path, Box::new(Router::new()));
                    Self::add_endpoint(router, endpoint, iter);
                } else {
                    // No Router & Iteration Ends
                    router.endpoints.push(endpoint);
                }
            }
            Some(PathType::Parameter(param)) => {
                if let Some(found_router) = &mut router.param_dir {
                    // Router Found
                    if iter.peek().is_some() {
                        Self::add_endpoint(found_router.1.as_mut(), endpoint, iter);
                    } else {
                        router.endpoints.push(endpoint);
                    }
                } else if iter.peek().is_some() {
                    // No Router & Iteration Continues
                    let mut inserted_router = Box::new(Router::new());
                    Self::add_endpoint(inserted_router.as_mut(), endpoint, iter);
                    router.param_dir = Some((param, inserted_router));
                } else {
                    // No Router & Iteration Ends
                    router.endpoints.push(endpoint);
                }
            }
            None => { panic!("Error: Route already exist.") }
        }
    }

    fn handle_routes<'a, 'b>(
        router: &'a Router,
        method: &HttpMethod,
        mut params: HashMap<String, String>,
        mut iter: std::slice::Iter<'_, String>
    ) -> RouterResult<'a> {
        if let Some(route) = iter.next() {
            if let Some(founded_router) = router.subdirs.get(route) {
                return Self::handle_routes(founded_router.as_ref(), method, params, iter);
            } else if let Some((param_name, founded_router)) = router.param_dir.as_ref() {
                params.insert(param_name.clone(), route.clone());
                return Self::handle_routes(founded_router, method, params, iter);
            } else {
                for endpoint in &router.endpoints {
                    if &endpoint.method == method {
                        return Some((endpoint, params.clone()));
                    }
                }
            }
        }

        return None;
    }

    fn get_path_types(path: &str) -> Vec<PathType> {
        path.split('/')
            .into_iter()
            .map(|path| {
                match PathType::try_from(path) {
                    Ok(path_type) => path_type,
                    Err(err) => panic!("Error while adding router: {}", err),
                }
            })
            .collect::<Vec<PathType>>()
    }
}

impl RouteHandler for Router {
    fn handle(&mut self, request: &Request, response: &mut Response) -> HandlerResult {
        while let Some(middleware) = self.middlewares.iter_mut().next() {
            match middleware.middleware(request, response) {
                HandlerResult::End => {
                    return HandlerResult::End;
                }
                HandlerResult::Next => (),
            }
        }

        match self.route_handler(request.get_path_array(), request.get_method()) {
            Some((endpoint, params)) => {
                // while let Some(middleware) = endpoint.middlewares.iter_mut().next() {
                //     match middleware.middleware(request, response) {
                //         HandlerResult::End => {
                //             return HandlerResult::End;
                //         }
                //         HandlerResult::Next => (),
                //     }
                // }

                (endpoint.controller)(request, response);
                return HandlerResult::Next;
            }
            None => {
                response.status(StatusCode::NotFound);
                return HandlerResult::Next;
            }
        }
    }
}

#[derive(Debug)]
pub struct Endpoint {
    method: HttpMethod,
    controller: Controller,
    middlewares: Vec<Box<dyn Middleware>>,
}

impl Endpoint {
    pub fn new(method: HttpMethod, controller: Controller) -> Self {
        Self {
            method,
            controller,
            middlewares: Vec::new(),
        }
    }

    pub fn new_with_middleware(
        method: HttpMethod,
        controller: Controller,
        middlewares: Vec<Box<dyn Middleware>>
    ) -> Self {
        Self {
            method,
            controller,
            middlewares,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum PathType {
    Subdirectory(String),
    Parameter(String),
}

impl TryFrom<&str> for PathType {
    type Error = ParsePathTypeError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with(':') {
            return Ok(Self::Parameter(value.chars().skip(1).collect::<String>()));
        } else {
            let regex = Regex::new(r"^[a-zA-Z0-9_\-.]+$").unwrap();

            if regex.is_match(value) {
                return Ok(Self::Subdirectory(value.to_string()));
            } else {
                return Err(ParsePathTypeError(value.to_string()));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsePathTypeError(String);

impl Display for ParsePathTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid path value for router: {}", self.0)
    }
}
