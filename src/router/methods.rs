use super::{ Controller, Endpoint, HttpMethod, Route, Router };

impl Router {
    /// Adds a GET endpoint to the router
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{router::Router, response::StatusCode};
    ///
    /// let mut router = Router::new();
    ///
    /// router.get("/", Box::new(|req, res| {
    ///    res.status(StatusCode::Ok);
    /// }));
    /// ```
        pub fn get(&mut self, path: &str, controller: Controller) {
        self.endpoints.insert(Route::new(path, &HttpMethod::GET), Endpoint::Controller(controller));
    }

    /// Adds a POST endpoint to the router
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{router::Router, response::StatusCode};
    ///
    /// let mut router = Router::new();
    ///
    /// router.post("/", Box::new(|req, res| {
    ///    res.status(StatusCode::Ok);
    /// }));
    /// ```
    pub fn post(&mut self, path: &str, controller: Controller) {
        self.endpoints.insert(
            Route::new(path, &HttpMethod::POST),
            Endpoint::Controller(controller)
        );
    }

    /// Adds a PUT endpoint to the router
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{router::Router, response::StatusCode};
    ///
    /// let mut router = Router::new();
    ///
    /// router.put("/", Box::new(|req, res| {
    ///    res.status(StatusCode::Ok);
    /// }));
    /// ```
    pub fn put(&mut self, path: &str, controller: Controller) {
        self.endpoints.insert(Route::new(path, &HttpMethod::PUT), Endpoint::Controller(controller));
    }

    /// Adds a DELETE endpoint to the router
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{router::Router, response::StatusCode};
    ///
    /// let mut router = Router::new();
    ///
    /// router.delete("/", Box::new(|req, res| {
    ///    res.status(StatusCode::Ok);
    /// }));
    /// ```
    pub fn delete(&mut self, path: &str, controller: Controller) {
        self.endpoints.insert(
            Route::new(path, &HttpMethod::DELETE),
            Endpoint::Controller(controller)
        );
    }

    /// Adds a PATCH endpoint to the router
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{router::Router, response::StatusCode};
    ///
    /// let mut router = Router::new();
    ///
    /// router.patch("/", Box::new(|req, res| {
    ///    res.status(StatusCode::Ok);
    /// }));
    /// ```
    pub fn patch(&mut self, path: &str, controller: Controller) {
        self.endpoints.insert(
            Route::new(path, &HttpMethod::PATCH),
            Endpoint::Controller(controller)
        );
    }
}
