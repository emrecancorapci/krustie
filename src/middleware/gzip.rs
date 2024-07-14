//! A middleware for compressing response body using gzip

use std::io::Write;
use flate2::{ write::GzEncoder, Compression };

use super::Middleware;
use crate::{ request::Request, response::Response, server::route_handler::HandlerResult };

/// A middleware for compressing response body using gzip.
///
/// # ExampleF
///
/// ```rust
/// use krustie::{server::Server, middleware::gzip::GzipEncoder};
///
/// let mut server = Server::create();
///
/// server.use_handler(GzipEncoder);
///
#[derive(Debug)]
pub struct GzipEncoder;

impl GzipEncoder {
    fn encode(body: &Vec<u8>) -> Result<Vec<u8>, String> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

        if encoder.write_all(body.as_slice()).is_err() {
            return Err("Error while writing to encoder".to_string());
        }

        match encoder.finish() {
            Ok(compressed_bytes) => {
                return Ok(compressed_bytes);
            }
            Err(err) => {
                return Err(format!("{err}"));
            }
        }
    }
}

impl Middleware for GzipEncoder {
    fn middleware(&self, request: &Request, response: &mut Response) -> HandlerResult {
        let body = response.get_body_mut();

        if body.is_empty() {
            return HandlerResult::Next;
        }

        if let Some(str_encodings) = request.get_header("accept-encoding") {
            let encodings = str_encodings
                .split(',')
                .map(|item| item.trim())
                .collect::<Vec<&str>>();

            if !encodings.contains(&"gzip") {
                return HandlerResult::Next;
            }

            match Self::encode(body) {
                Ok(compressed_bytes) => {
                    response.insert_header("Content-Encoding", "gzip");

                    let _ = response.update_body(compressed_bytes);
                }
                Err(err) => {
                    eprintln!("Error while compressing: {}", err);
                }
            }
        }

        return HandlerResult::Next;
    }
}
