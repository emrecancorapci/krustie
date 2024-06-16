use std::{ io::{ BufRead, BufReader, Read }, net::TcpStream };

use super::HttpRequest;

impl HttpRequest {
    /// Parses a TcpStream into HttpRequest
    pub fn parse_stream(mut stream: &TcpStream) -> Result<HttpRequest, String> {
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

        match HttpRequest::get_content_length(&headers) {
            Some(content_length) => {
                let mut body = Vec::with_capacity(content_length);

                match buf_reader.take(content_length as u64).read_to_end(&mut body) {
                    Ok(_) => {
                        match String::from_utf8(body) {
                            Ok(body) => { Ok(HttpRequest::new(&headers, body.as_str())) }
                            Err(_) => { Err("Error while parsing body".to_string()) }
                        }
                    }
                    Err(error) => { Err(format!("Error while reading body: {}", error.to_string())) }
                }
            }
            None => { Ok(HttpRequest::new(&headers, "")) }
        }
    }

    /// Gets the content length from the headers
    fn get_content_length(headers: &Vec<String>) -> Option<usize> {
        for line in headers.iter() {
            if line.starts_with("Content-Length") {
                let parts = line.split(':').collect::<Vec<&str>>();

                match parts[1].trim().parse::<usize>() {
                    Ok(parsed) => {
                        return Some(parsed);
                    }
                    Err(_) => {
                        return None;
                    }
                }
            }
        }

        return None;
    }
}
