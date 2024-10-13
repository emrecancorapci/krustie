use krustie::{HttpMethod, Request, Response, Router, Server, StatusCode};

#[test]
fn router_parameters() {
    // Create a new server instance
    let mut server = Server::create();
    let mut router = Router::new();

    router.get("/echo/:param", |req, res| {
        res.status(StatusCode::Ok)
            .body_text(req.get_param("param").unwrap());
    });

    server.use_handler(router);

    // Test the router
    let mut expected_response = Response::default();
    expected_response.status(StatusCode::Ok).body_text("hello");

    let request = Request::builder()
        .method(HttpMethod::GET)
        .path("/echo/hello")
        .build();

    let response = server.mock_request(request);

    Response::assert_eq(&expected_response, &response);
}

#[test]
fn query_parameters() {
    // Create a new server instance
    let mut server = Server::create();
    let mut router = Router::new();

    router.get("/echo", |req, res| {
        res.status(StatusCode::Ok)
            .body_text(req.get_query_param("query").unwrap());
    });

    server.use_handler(router);

    // Test the router
    let mut expected_response = Response::default();
    expected_response.status(StatusCode::Ok).body_text("world");

    let request = Request::builder()
        .method(HttpMethod::GET)
        .path("/echo?query=world")
        .build();

    let response = server.mock_request(request);

    Response::assert_eq(&expected_response, &response);
}

#[test]
fn router_and_query_parameters() {
    // Create a new server instance
    let mut server = Server::create();
    let mut router = Router::new();

    router.get("/echo/:param", |req, res| {
        res.status(StatusCode::Ok).body_text(
            format!(
                "{} {}",
                req.get_param("param").unwrap(),
                req.get_query_param("query").unwrap()
            )
            .as_str(),
        );
    });

    server.use_handler(router);

    // Test the router
    let mut expected_response = Response::default();
    expected_response
        .status(StatusCode::Ok)
        .body_text("hello world");

    let request = Request::builder()
        .method(HttpMethod::GET)
        .path("/echo/hello?query=world")
        .build();

    let response = server.mock_request(request);

    Response::assert_eq(&expected_response, &response);
}
