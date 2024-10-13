use super::Response;

impl Response {
    pub fn assert_eq(resp1: &Response, resp2: &Response) {
        assert_eq!(&resp1.get_status(), &resp2.get_status());
        let _ = &resp1.get_headers().iter().for_each(|(key, value)| {
            assert_eq!(resp2.get_headers().get(key), Some(value));
        });
        assert_eq!(&resp1.get_body(), &resp2.get_body());
    }
}
