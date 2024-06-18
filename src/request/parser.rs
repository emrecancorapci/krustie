use std::{ io::{ BufRead, BufReader, Read }, net::TcpStream };

use super::HttpRequest;

pub(crate) trait Parse {
    fn parse(stream: &TcpStream) -> Result<HttpRequest, String>;
}

impl Parse for HttpRequest {
    /// Parses a TcpStream into HttpRequest
    fn parse(mut stream: &TcpStream) -> Result<Self, String> {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut headers = Vec::new();

        // Don't touch this. It's too sensitive :(((.
        for line_result in buf_reader.by_ref().lines() {
            let line = line_result.unwrap();
            if line.is_empty() {
                break;
            }
            headers.push(line);
        }

        match parse_length(&headers) {
            None => { Self::new(&headers, None).map_err(|err| err.to_string()) }
            Some(0) => { Self::new(&headers, None).map_err(|err| err.to_string()) }
            Some(content_length) => {
                let mut body = Vec::with_capacity(content_length);

                if
                    buf_reader
                        .take(content_length as u64)
                        .read_to_end(&mut body)
                        .is_err()
                {
                    return Err("Error while reading body".to_string());
                }

                match String::from_utf8(body) {
                    Ok(body) => {
                        HttpRequest::new(&headers, Some(body.as_str())).map_err(|err|
                            err.to_string()
                        )
                    }
                    Err(_) => { Err("Error while parsing body".to_string()) }
                }
            }
        }
    }
}

/// Gets the content length from the headers
fn parse_length(headers: &Vec<String>) -> Option<usize> {
    for line in headers.iter() {
        if line.starts_with("Content-Length") {
            match line.find(":") {
                Some(index) => {
                    return Some(line[index + 1..].trim().parse::<usize>().unwrap_or(0));
                }
                None => {
                    return None;
                }
            }
        }
    }
    return None;
}
