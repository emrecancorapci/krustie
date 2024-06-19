use crate::{ request::HttpRequest, response::HttpResponse, server::Handler };

pub mod gzip;

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
///   fn middleware(req: &HttpRequest, res: &mut HttpResponse) {
///     AddKrustieHeader::add_header(res);
///   }
/// }
///
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