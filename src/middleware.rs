use crate::{
    request::HttpRequest,
    response::HttpResponse,
    server::route_handler::{ RouteHandler, HandlerResult },
};

pub mod gzip;
pub mod statics;

pub use self::{ gzip::GzipEncoder, statics::ServeStaticFiles };

/// Middleware trait to be implemented for creating middleware.
/// 
/// If there is no property declared in the struct, struct can be used directly.
/// Or it can be used as a value if it needs to be initialized.
/// 
/// # Example
/// 
/// - In this example `AddKrustieHeader` can be used as `server.add_handler(AddKrustieHeader)`
///
/// ```rust
/// use krustie::{ HttpRequest, HttpResponse, Middleware, server::route_handler::HandlerResult };
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
///   fn middleware(&self, req: &HttpRequest, res: &mut HttpResponse) -> HandlerResult {
///     AddKrustieHeader::add_header(res);
///     HandlerResult::Next
///   }
/// }
/// ```
/// 
/// - In this example `AddHeader` should be initialized.
/// 
/// ```rust
/// use krustie::{ HttpRequest, HttpResponse, Middleware, server::route_handler::HandlerResult };
///
/// struct AddHeader {
///     server: String,
///     value: String,
/// }
/// 
/// impl AddHeader {
///     fn new(server: &str, value: &str) -> Self {
///         Self { server: server.to_string(), value: value.to_string() }
///     }
/// }
/// 
/// impl Middleware for AddHeader {
///   fn middleware(&self, _: &HttpRequest, res: &mut HttpResponse) -> HandlerResult {
///     res.insert_header(&self.server, &self.value);
///     HandlerResult::Next
///   }
/// }
/// ```
///
pub trait Middleware {
    fn middleware(&self, req: &HttpRequest, res: &mut HttpResponse) -> HandlerResult;
}

impl<T> RouteHandler for T where T: Middleware {
    fn handle(
        &self,
        request: &HttpRequest,
        response: &mut HttpResponse,
        _: &Vec<String>
    ) -> HandlerResult {
        T::middleware(&self, request, response)
    }
}
