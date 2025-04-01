use std::{collections::HashMap, io::Result, net::SocketAddr, sync::Arc};

use async_trait::async_trait;

pub trait HttpStatusCode: Sized + Eq + PartialEq {
    fn code(&self) -> u16;
    fn msg(&self) -> &str;
    fn is_err(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE,
}

#[async_trait]
pub trait HttpRequest: Send + Sized + Sync {
    async fn new(msg: &[u8], peer_addr: SocketAddr) -> Result<Self>;

    fn get_method(&self) -> &HttpMethod;
    fn get_path(&self) -> &str;
    fn get_version(&self) -> &str;
    fn get_body(&self) -> &[u8];

    fn get_headers(&self) -> &HashMap<String, String>;
    fn get_header(&self, key: &str) -> Option<&str>;

    fn get_query_params(&self) -> &HashMap<String, String>;
    fn get_query_param(&self, key: &str) -> Option<&str>;
    fn get_param(&self, key: &str) -> Option<&str>;

    fn get_cookies(&self) -> &HashMap<String, String>;
    fn get_cookie(&self, key: &str) -> Option<&str>;
    fn get_content_type(&self) -> Option<&str>;
    fn get_user_agent(&self) -> Option<&str>;
    fn accepts(&self, content_type: &str) -> bool;

    async fn parse_form_data(&self) -> HashMap<String, String>;

    fn is_multipart(&self) -> bool;
    fn get_multipart_fields(&self) -> HashMap<String, String>;
    fn get_multipart_field(&self, name: &str) -> Option<&str>;
    fn get_multipart_files(&self) -> HashMap<String, Vec<u8>>;

    fn get_peer_addr(&self) -> &SocketAddr;
}

pub trait HttpResponse: Send + Sized + ToString + Into<Vec<u8>> {
    fn new() -> Self;

    fn set_status(&mut self, status_code: impl HttpStatusCode) -> &mut Self;
    fn set_header(&mut self, key: &str, value: &str) -> &mut Self;
    fn set_body(&mut self, body: Vec<u8>, mime: &str) -> &mut Self;
    fn set_body_text(&mut self, text: &str) -> &mut Self;
    fn send_file(&mut self, path: &str) -> Result<&mut Self>;
    fn extend_headers(&mut self, headers: HashMap<String, String>) -> &mut Self;

    fn get_status(&self) -> &impl HttpStatusCode;
    fn get_header(&self, key: &str) -> Option<&String>;
    fn get_headers(&self) -> &HashMap<String, String>;
    fn get_body(&self) -> &Vec<u8>;

    fn get_headers_mut(&mut self) -> &mut HashMap<String, String>;
    fn get_body_mut(&mut self) -> &mut Vec<u8>;
    fn get_header_mut(&mut self, key: &str) -> Option<&mut String>;

    fn set_cookie(&mut self, key: &str, value: &str) -> &mut Self;
    fn get_cookie(&self, key: &str) -> Option<&str>;
    fn remove_cookie(&mut self, key: &str) -> &mut Self;

    fn redirect(&mut self, url: &str, status_code: Option<u16>) -> &mut Self;

    fn get_local(&self, key: &str) -> Option<&String>;
    fn set_local(&mut self, key: &str, value: &str) -> Option<String>;
}

#[async_trait]
pub trait RouteHandling<Req, Res, Ctx>: Sync + Send
where
    Req: HttpRequest,
    Res: HttpResponse,
    Ctx: Context,
{
    async fn handle(&self, request: &mut Req, response: Res, context: Arc<Ctx>) -> Res;
    fn use_router(&self, path: String, router: Self);
}

pub trait Routing<Req: HttpRequest, Res: HttpResponse, Ctx: Context>:
    RouteHandling<Req, Res, Ctx> + Sync + Send
{
    fn get_params(&self, path: &str) -> HashMap<String, String>;
}

pub trait Context: Sync + Send {}
