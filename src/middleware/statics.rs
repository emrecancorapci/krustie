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

use std::{ collections::HashMap, fs, path::PathBuf };

use crate::{ server::route_handler::HandlerResult, Request, Response, Middleware, StatusCode };

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
pub struct ServeStaticFiles {
    folder_path: String,
}

impl ServeStaticFiles {
    /// Creates a new instance of ServeStaticFiles
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::middleware::ServeStaticFiles;
    ///
    /// let statics = ServeStaticFiles::new("public");
    /// ```
    pub fn new(folder_path: &str) -> ServeStaticFiles {
        ServeStaticFiles {
            folder_path: folder_path.to_string(),
        }
    }

    fn get_extension(&self, path: &PathBuf) -> Result<String, String> {
        match path.extension() {
            Some(ext) => {
                match ext.to_str() {
                    Some(val) => Ok(val.to_string()),
                    None => {
                        return Err(format!("Failed to convert extension to string: {:?}", ext));
                    }
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

        let content_type = ContentType::from(extension.as_str());

        match fs::read(&path) {
            Ok(content) => {
                response.status(StatusCode::Ok).body(content, content_type);
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
