use crate::{ HttpRequest, HttpResponse };

pub trait RouteHandler {
    fn handle(&self, request: &HttpRequest, response: &mut HttpResponse, path: &Vec<String>) -> HandlerResult;
}

#[derive(Debug, PartialEq, Eq)]
pub enum HandlerResult {
    End,
    Next,
}