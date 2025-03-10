use std::{collections::HashMap, net::SocketAddr};

use async_trait::async_trait;

use crate::http::core::{HttpMethod, HttpRequest};

pub struct Request;

#[async_trait]
impl HttpRequest for Request {
    async fn new(_: &[u8], __: SocketAddr) -> Result<Self, std::io::Error> {
        todo!()
    }

    fn get_method(&self) -> HttpMethod {
        todo!()
    }

    fn get_path(&self) -> &str {
        todo!()
    }

    fn get_version(&self) -> &str {
        todo!()
    }

    fn get_body(&self) -> &[u8] {
        todo!()
    }

    fn get_headers(&self) -> &HashMap<String, String> {
        todo!()
    }

    fn get_header(&self, _key: &str) -> Option<&str> {
        todo!()
    }

    fn get_query_params(&self) -> &HashMap<String, String> {
        todo!()
    }

    fn get_query_param(&self, _key: &str) -> Option<&str> {
        todo!()
    }

    fn get_param(&self, _key: &str) -> Option<&str> {
        todo!()
    }

    fn get_cookies(&self) -> HashMap<String, String> {
        todo!()
    }

    fn get_cookie(&self, _key: &str) -> Option<&str> {
        todo!()
    }

    fn get_content_type(&self) -> Option<&str> {
        todo!()
    }

    fn get_user_agent(&self) -> Option<&str> {
        todo!()
    }

    fn accepts(&self, _content_type: &str) -> bool {
        todo!()
    }

    async fn parse_form_data(&self) -> HashMap<String, String> {
        todo!()
    }

    fn is_multipart(&self) -> bool {
        todo!()
    }

    fn get_multipart_fields(&self) -> HashMap<String, String> {
        todo!()
    }

    fn get_multipart_field(&self, _name: &str) -> Option<&str> {
        todo!()
    }

    fn get_multipart_files(&self) -> HashMap<String, Vec<u8>> {
        todo!()
    }

    fn get_peer_addr(&self) -> &SocketAddr {
        todo!()
    }
}
