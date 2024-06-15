use crate::{ request::HttpRequest, response::HttpResponse, server::Handler };

type MiddlewareFn = fn(&HttpRequest, &mut HttpResponse);

pub struct Middleware {
    middleware: MiddlewareFn,
}

impl Middleware {
    pub fn new(middleware: MiddlewareFn) -> Middleware {
        Middleware {
            middleware,
        }
    }
}

impl Handler for Middleware {
    fn handle(&self, req: &HttpRequest, res: &mut HttpResponse) {
        (self.middleware)(req, res);
    }
}