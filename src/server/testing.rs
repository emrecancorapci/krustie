//! Testing utilities for the server

use crate::{Request, Response};

use super::{route_handler::HandlerResult, Server};

impl Server {
    /// Mocks a request and returns a response
    ///
    /// This method is useful for testing the server without actually listening on a port.
    pub fn mock_request(&mut self, request: Request) -> Response {
        let mut response = Response::default();

        for handler in &mut self.route_handlers {
            let result = handler.handle(&request, &mut response);
            if result == HandlerResult::End {
                break;
            }
        }

        response
    }

    /// Mocks a request and expects a specific response
    ///
    /// This method is useful for testing the server without actually listening on a port.
    pub fn mock_request_and_expect(&mut self, request: Request, expected_response: Response) {
        let mut response = Response::default();

        for handler in &mut self.route_handlers {
            let result = handler.handle(&request, &mut response);
            if result == HandlerResult::End {
                break;
            }
        }

        Response::assert_eq(&response, &expected_response);
    }
}
