use crate::{ request::HttpRequest, response::HttpResponse, server::Handler };

pub mod gzip;

pub trait Middleware {
    fn middleware(req: &HttpRequest, res: &mut HttpResponse) where Self: Sized;
}

pub trait MiddlewareHandler: Middleware + Handler {}

// Implement the `Handler` trait for any type that implements the `Middleware` trait
impl<T> Handler for T
where
    T: Middleware,
{
    fn handle(&self, request: &HttpRequest, response: &mut HttpResponse) {
        T::middleware(request, response);
    }
}