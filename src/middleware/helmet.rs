use crate::{ Request, Response };


pub mod policies;

struct Helmet {}

impl Helmet {
    fn origin_access_cluster(&self) -> impl Fn(&Request, &mut Response) {
        |_: &Request, response: &mut Response| {
            response.insert_header("Origin-Agent-Cluster", "?1");
        }
    }

    fn strict_transport_security(&self, options: StrictTransportSecuirtyOptions) {
        
    }
}

struct StrictTransportSecuirtyOptions {
    max_age: u128,
    include_sub_domains: bool,
    directives: bool,
}