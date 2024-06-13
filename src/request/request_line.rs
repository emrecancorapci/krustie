use super::HttpMethod;

pub struct RequestLine<'a> {
    pub method: HttpMethod,
    pub path: &'a str,
    pub version: &'a str,
    pub path_array: Vec<&'a str>,
}

impl<'a> RequestLine<'a> {
    pub fn new(method: &'a str, path: &'a str, version: &'a str) -> RequestLine<'a> {
        let path_array: Vec<&str> = path[1..].split('/').collect();
        let method = HttpMethod::new(method).expect("Method not found");

        RequestLine {
            method,
            path,
            version,
            path_array,
        }
    }

    pub fn new_from_str(request_line: &str) -> Result<RequestLine, &'static str> {
        let request_line: Vec<&str> = request_line.split(' ').collect();

        if request_line.len() < 3 {
            return Err("request_line does not have 3 parts");
        }

        Ok(RequestLine::new(&request_line[0], &request_line[1], &request_line[2]))
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.method, self.path, self.version)
    }
}
