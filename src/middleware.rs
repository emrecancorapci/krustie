use crate::{ request::HttpRequest, response::HttpResponse, server::Handler };

pub mod gzip;

type MiddlewareFn = fn(&HttpRequest, &mut HttpResponse);

pub trait Middleware {
    fn middleware(req: &HttpRequest, res: &mut HttpResponse);
    fn get_middleware() -> MiddlewareHandler {
        MiddlewareHandler::new(Self::middleware)
    }
}

pub struct MiddlewareHandler {
    middleware: MiddlewareFn,
}

impl MiddlewareHandler {
    pub fn new(middleware: MiddlewareFn) -> Self {
        Self {
            middleware,
        }
    }
}

impl Handler for MiddlewareHandler {
    fn handle(&self, req: &HttpRequest, res: &mut HttpResponse) {
        (self.middleware)(req, res);
    }
}
