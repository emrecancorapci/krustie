use std::{ io::{ BufRead, BufReader, Read }, net::TcpStream };

use crate::server::Server;

impl Server {
    /// Parses a TcpStream into headers and body
    pub fn parse_stream(mut stream: &TcpStream) -> Result<(Vec<String>, String), String> {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut headers = Vec::new();

        // Don't touch this. It's too sensitive. It will break the server.
        for line_result in buf_reader.by_ref().lines() {
            let line = line_result.unwrap();
            if line.is_empty() {
                break;
            }
            headers.push(line);
        }

        match Server::get_content_length(&headers) {
            Some(content_length) => {
                let mut body = Vec::with_capacity(content_length);

                match buf_reader.take(content_length as u64).read_to_end(&mut body) {
                    Ok(_) => {
                        match String::from_utf8(body) {
                            Ok(body) => { Ok((headers, body)) }
                            Err(_) => { Err("Error parsing body".to_string()) }
                        }
                    }
                    Err(error) => { Err(format!("Error reading body: {}", error.to_string())) }
                }
            }
            None => { Ok((headers, String::from(""))) }
        }
    }

    /// Gets the content length from the headers
    fn get_content_length(headers: &Vec<String>) -> Option<usize> {
        let mut content_length = 0;

        for line in headers.iter() {
            if line.starts_with("Content-Length") {
                let parts = line.split(':').collect::<Vec<&str>>();

                match parts[1].trim().parse::<usize>() {
                    Ok(parsed) => {
                        content_length = parsed;
                    }
                    Err(_) => {
                        content_length = 0;
                    }
                }
                break;
            }
        }

        if content_length == 0 {
            return None;
        }

        Some(content_length)
    }
}
