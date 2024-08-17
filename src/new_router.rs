use std::{ collections::HashMap, fmt::Display };

use regex::Regex;

use crate::{ HttpMethod, Middleware, Request, Response };

type Controller = fn(&Request, &mut Response);

#[derive(Debug)]
pub struct Router {
    endpoints: HashMap<PathType, Endpoint>,
    middlewares: Vec<Box<dyn Middleware>>,
    subdirs: HashMap<String, Box<Router>>,
    param_dir: Option<(String, Box<Router>)>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            endpoints: HashMap::new(),
            middlewares: Vec::new(),
            subdirs: HashMap::new(),
            param_dir: None,
        }
    }

    pub fn add_router(&mut self, path: &str, router: Router) {
        if path == "/" || path.is_empty() {
            panic!("Can't add router to empty path.");
        }

        let path_types = path
            .split('/')
            .into_iter()
            .map(|path| {
                match PathType::try_from(path) {
                    Ok(path_type) => path_type,
                    Err(err) => panic!("Error while adding router: {}", err),
                }
            })
            .collect::<Vec<PathType>>();

        let mut types_iter = path_types.into_iter().peekable();

        create_router(self, router, &mut types_iter);
    }

    pub fn add_endpoint(&mut self, path: &str, method: HttpMethod, controller: Controller) {
        let path_type = PathType::try_from(path).unwrap_or_else(|err| panic!("{}", err));
        self.endpoints.insert(path_type, Endpoint::new(method, controller, Vec::new()));
    }

    pub fn add_endpoint_with_middleware(
        &mut self,
        path: &str,
        method: HttpMethod,
        controller: Controller,
        middlewares: Vec<Box<dyn Middleware>>
    ) {
        let path_type = PathType::try_from(path).unwrap_or_else(|err| panic!("{}", err));
        self.endpoints.insert(path_type, Endpoint::new(method, controller, middlewares));
    }

    fn built_route(&self, path: &str) {
        let mut current_router = self;
        let parameters: HashMap<String, String> = HashMap::new();

        let split_path = path.split('/');
        let mut peekable = split_path.peekable();

        while let Some(current) = peekable.next() {
            // api - users - 23?take=username
            // There is next one -> Look subdirectories
            // Find api subdirectory add middlewares
            // There is next one -> Look subdirectories
            // Find users subdirectory add middlewares
            // There is next one -> Look subdirectories -> NOT FOUND
            // Is there parameter endpoint - YES
            // Take controller
            // Is there '?' in last path -> YES
            // Take queries and send

            if peekable.peek() != None {
                let subdir = current_router.subdirs.get(current);

                if subdir.is_some() {
                    current_router = subdir.unwrap();
                    continue;
                }

                if current_router.param_dir.is_some() {
                    current_router = current_router.param_dir.as_ref().unwrap().1.as_ref();
                } else {
                    todo!();
                }
            } else {
                let endpoints_iter = self.endpoints.values().enumerate().into_iter();
            }
        }
    }
}

struct BuiltRoute {
    controller: Controller,
    parameters: HashMap<String, String>,
    queries: HashMap<String, String>,
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
            let regex = Regex::new(r"^[a-zA-Z0-9_-.]+$").unwrap();

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
