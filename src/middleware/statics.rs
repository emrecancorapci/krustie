//! Static files middleware.
//!
//! This middleware serves static files from a specified folder.
//!
//! ## Supported Content Types
//!
//! - **Document:** `html`, `css`, `js`, `json`, `xml`
//! - **Image:** `png`, `jpg`, `jpeg`, `gif`, `svg`, `ico`
//! - **File:** `pdf`, `zip`, `gzip`
//! - **Media:** `mp3`, `wav`, `mp4`, `mpeg`, `webm`
//! - **Font:** `woff`, `woff2`, `ttf`, `otf`, `eot`

use std::{ fs, path::PathBuf };

use crate::{
    response::content_type::ContentType,
    server::route_handler::HandlerResult,
    Middleware,
    Request,
    Response,
    StatusCode,
};

#[derive(Debug)]
/// Serve static files from a specified folder.
///
/// # Example
///
/// ```rust
/// use krustie::{ Server, middleware::ServeStatic };
///
/// let mut server = Server::create();
/// let statics = ServeStatic::new("public");
///
/// server.use_handler(statics);
/// ```
pub struct ServeStatic {
    folder_path: String,
}

impl ServeStatic {
    /// Creates a new instance of ServeStatic
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::middleware::ServeStatic;
    ///
    /// let statics = ServeStatic::new("public");
    /// ```
    pub fn new(folder_path: &str) -> ServeStatic {
        ServeStatic {
            folder_path: folder_path.to_string(),
        }
    }

    fn get_extension(&self, path: &PathBuf) -> Result<String, String> {
        match path.extension() {
            Some(ext) =>
                match ext.to_str() {
                    Some(val) => Ok(val.to_string()),
                    None => {
                        return Err(format!("Failed to convert extension to string: {:?}", ext));
                    }
                }
            None => {
                return Err(format!("No extension found for file: {:?}", path));
            }
        }
    }
}

impl Middleware for ServeStatic {
    fn middleware(&self, request: &Request, response: &mut Response) -> HandlerResult {
        let file_name = &request.get_path_array()[0];

        let path = PathBuf::from(&self.folder_path).join(file_name);
        let extension = match self.get_extension(&path) {
            Ok(ext) => ext,
            Err(err) => {
                eprintln!("{}", err);
                return HandlerResult::Next;
            }
        };

        let content_type = ContentType::try_from(extension.as_str());

        if content_type.is_err() {
            response.status(StatusCode::UnsupportedMediaType);
            return HandlerResult::End;
        }

        match fs::read(&path) {
            Ok(content) => {
                response.status(StatusCode::Ok).body(content, content_type.unwrap());
                return HandlerResult::End;
            }
            Err(_) => {
                eprintln!("Failed to read file: {:?}", path);
                return HandlerResult::Next;
            }
        }
    }
}

impl Default for ServeStatic {
    fn default() -> Self {
        Self::new("public")
    }
}
