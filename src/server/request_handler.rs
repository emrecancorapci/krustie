use crate::{request::HttpRequest, response::HttpResponse, router::Router};


pub type Middleware = Box<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync>;

pub enum RequestHandler {
    Router(Router),
    Middleware(Middleware),
}

impl RequestHandler {
    pub fn run(&self, request: &HttpRequest, response: &mut HttpResponse) {
        match self {
            RequestHandler::Middleware(middleware) => {
                middleware(request, response);
            }
            RequestHandler::Router(router) => {
                router.handle(request, response);
            }
        }
    }
}
