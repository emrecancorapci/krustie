use super::{ Controller, HttpMethod, Router };

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
    /// router.get(|req, res| {
    ///    res.status(StatusCode::Ok);
    /// });
    /// ```
    pub fn get(&mut self, controller: Controller) -> &mut Self {
        self.endpoints.insert(HttpMethod::GET, controller);
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
    /// router.post(|req, res| {
    ///   res.status(StatusCode::Ok);
    /// });
    /// ```
    pub fn post(&mut self, controller: Controller) -> &mut Self {
        self.endpoints.insert(HttpMethod::POST, controller);
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
    /// router.put(|req, res| {
    ///   res.status(StatusCode::Ok);
    /// });
    /// ```
    pub fn put(&mut self, controller: Controller) -> &mut Self {
        self.endpoints.insert(HttpMethod::PUT, controller);
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
    /// router.delete(|req, res| {
    ///   res.status(StatusCode::Ok);
    /// });
    /// ```
    pub fn delete(&mut self, controller: Controller) -> &mut Self {
        self.endpoints.insert(HttpMethod::DELETE, controller);
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
    /// router.patch(|req, res| {
    ///   res.status(StatusCode::Ok);
    /// });
    /// ```
    pub fn patch(&mut self, controller: Controller) -> &mut Self {
        self.endpoints.insert(HttpMethod::PATCH, controller);
        self
    }
}
