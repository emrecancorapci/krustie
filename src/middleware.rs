use crate::{
    request::HttpRequest,
    response::HttpResponse,
    server::route_handler::{ RouteHandler, HandlerResult },
};

pub mod gzip;

pub use gzip::Gzip;

/// Middleware trait to be implemented for creating middleware
///
/// # Example
///
/// ```rust
/// use krustie::{ request::HttpRequest, response::HttpResponse, middleware::Middleware };
///
/// struct AddKrustieHeader;
///
/// impl AddKrustieHeader {
///   fn add_header(res: &mut HttpResponse) {
///     res.insert_header("Server", "Krustie");
///   }
/// }
///
/// impl Middleware for AddKrustieHeader {
///   fn middleware(&self, req: &HttpRequest, res: &mut HttpResponse) {
///     AddKrustieHeader::add_header(res);
///   }
/// }
///
pub trait Middleware {
    fn middleware(&self, req: &HttpRequest, res: &mut HttpResponse) -> HandlerResult;
}

impl RouteHandler for Box<dyn Middleware> {
    fn handle(
        &self,
        request: &HttpRequest,
        response: &mut HttpResponse,
        _: &Vec<String>
    ) -> HandlerResult {
        self.middleware(request, response)
    }
}
