use crate::{ request::HttpRequest, response::HttpResponse, server::Handler };

pub mod gzip;

pub trait Middleware {
    fn middleware(req: &HttpRequest, res: &mut HttpResponse);
    fn get_middleware() -> MiddlewareHandler {
        MiddlewareHandler {
            middleware: Self::middleware,
        }
    }
}

pub struct MiddlewareHandler {
    middleware: fn(&HttpRequest, &mut HttpResponse),
}

impl Handler for MiddlewareHandler {
    fn handle(&self, req: &HttpRequest, res: &mut HttpResponse) {
        (self.middleware)(req, res);
    }
}
