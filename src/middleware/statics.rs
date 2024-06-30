use std::{ collections::HashMap, fs, path::PathBuf };

use crate::{
    server::route_handler::HandlerResult,
    HttpRequest,
    HttpResponse,
    Middleware,
    StatusCode,
};

#[derive(Debug)]
pub struct ServeStaticFiles {
    folder_path: String,
    content_types: HashMap<String, String>,
}

impl ServeStaticFiles {
    pub fn new(folder_path: &str) -> ServeStaticFiles {
        let mut content_types = HashMap::new();
        let extensions = vec![
            ("html", "text/html"),
            ("css", "text/css"),
            ("js", "text/javascript"),
            ("png", "image/png"),
            ("jpg", "image/jpg"),
            ("jpeg", "image/jpeg"),
            ("gif", "image/gif"),
            ("svg", "image/svg+xml"),
            ("ico", "image/x-icon"),
            ("json", "application/json"),
            ("pdf", "application/pdf"),
            ("xml", "application/xml"),
            ("zip", "application/zip"),
            ("gzip", "application/gzip"),
            ("mp3", "audio/mpeg"),
            ("wav", "audio/wav"),
            ("mp4", "video/mp4"),
            ("mpeg", "video/mpeg"),
            ("webm", "video/webm"),
            ("woff", "font/woff"),
            ("woff2", "font/woff2"),
            ("ttf", "font/ttf"),
            ("otf", "font/otf"),
            ("eot", "font/eot"),
            ("svg", "font/svg")
        ];

        for (ext, content_type) in extensions {
            content_types.insert(ext.to_string(), content_type.to_string());
        }

        ServeStaticFiles {
            folder_path: folder_path.to_string(),
            content_types,
        }
    }

    pub fn add_content_type(&mut self, extension: &str, content_type: &str) {
        self.content_types.insert(extension.to_string(), content_type.to_string());
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

impl Middleware for ServeStaticFiles {
    fn middleware(&self, request: &HttpRequest, response: &mut HttpResponse) -> HandlerResult {
        let file_name = &request.get_path_array()[0];

        let path = PathBuf::from(&self.folder_path).join(file_name);
        let extension = match self.get_extension(&path) {
            Ok(ext) => ext,
            Err(err) => {
                eprintln!("{}", err);
                return HandlerResult::Next;
            }
        };

        let content_type = match self.content_types.get(extension.as_str()) {
            Some(content_type) => content_type,
            None => {
                return HandlerResult::Next;
            }
        };

        match fs::read(path) {
            Ok(content) => {
                response.status(StatusCode::Ok).body(content, content_type);
                return HandlerResult::End;
            }
            Err(_) => {
                return HandlerResult::Next;
            }
        }
    }
}
