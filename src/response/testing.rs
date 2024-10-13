//! This module contains testing utilities for the response module.

use super::Response;

impl Response {
    /// Compares two responses and asserts that they are equal
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::Response;
    ///
    /// let response1 = Response::default();
    /// let response2 = Response::default();
    ///
    /// Response::assert_eq(&response1, &response2);
    /// ```
    pub fn assert_eq(resp1: &Response, resp2: &Response) {
        assert_eq!(&resp1.get_status(), &resp2.get_status());
        let _ = &resp1.get_headers().iter().for_each(|(key, value)| {
            assert_eq!(resp2.get_headers().get(key), Some(value));
        });
        assert_eq!(&resp1.get_body(), &resp2.get_body());
    }
}
