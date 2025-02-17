use super::{Controller, Endpoint, HttpMethod, Router};

impl Router {
    /// Adds a GET endpoint to the router
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Router, StatusCode };
    ///
    /// let mut router = Router::new();
    ///
    /// router.get("/", |req, res| {
    ///    res.status(StatusCode::Ok);
    /// });
    /// ```
    pub fn get(&mut self, path: &str, controller: Controller) -> &mut Self {
        let endpoint = Endpoint::new(HttpMethod::GET, controller);
        self.use_endpoint(path, endpoint);
        self
    }

    /// Adds a POST endpoint to the router
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Router, StatusCode };
    ///
    /// let mut router = Router::new();
    ///
    /// router.post("/", |req, res| {
    ///   res.status(StatusCode::Ok);
    /// });
    /// ```
    pub fn post(&mut self, path: &str, controller: Controller) -> &mut Self {
        let endpoint = Endpoint::new(HttpMethod::POST, controller);
        self.use_endpoint(path, endpoint);
        self
    }

    /// Adds a PUT endpoint to the router
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Router, StatusCode };
    ///
    /// let mut router = Router::new();
    ///
    /// router.put("/", |req, res| {
    ///   res.status(StatusCode::Ok);
    /// });
    /// ```
    pub fn put(&mut self, path: &str, controller: Controller) -> &mut Self {
        let endpoint = Endpoint::new(HttpMethod::PUT, controller);
        self.use_endpoint(path, endpoint);
        self
    }

    /// Adds a DELETE endpoint to the router
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Router, StatusCode };
    ///
    /// let mut router = Router::new();
    ///
    /// router.delete("/", |req, res| {
    ///   res.status(StatusCode::Ok);
    /// });
    /// ```
    pub fn delete(&mut self, path: &str, controller: Controller) -> &mut Self {
        let endpoint = Endpoint::new(HttpMethod::DELETE, controller);
        self.use_endpoint(path, endpoint);
        self
    }

    /// Adds a PATCH endpoint to the router
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Router, StatusCode };
    ///
    /// let mut router = Router::new();
    ///
    /// router.patch("/", |req, res| {
    ///   res.status(StatusCode::Ok);
    /// });
    /// ```
    pub fn patch(&mut self, path: &str, controller: Controller) -> &mut Self {
        let endpoint = Endpoint::new(HttpMethod::PATCH, controller);
        self.use_endpoint(path, endpoint);
        self
    }
}
