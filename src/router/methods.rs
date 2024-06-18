use super::{ Controller, HttpMethod, Router };

pub trait Endpoints<TController> {
    fn get(&mut self, controller: TController) -> &mut Self;
    fn post(&mut self, controller: TController) -> &mut Self;
    fn put(&mut self, controller: TController) -> &mut Self;
    fn delete(&mut self, controller: TController) -> &mut Self;
    fn patch(&mut self, controller: TController) -> &mut Self;

}

impl Endpoints<Controller> for Router {
    /// Adds a GET endpoint to the router
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{router::Router, response::StatusCode};
    ///
    /// let mut router = Router::new();
    ///
    /// router.get(|req, res| {
    ///    res.status(StatusCode::Ok);
    /// });
    /// ```
    fn get(&mut self, controller: Controller) -> &mut Self {
        self.endpoints.insert(HttpMethod::GET, controller);
        self
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
    /// router.post(|req, res| {
    ///    res.status(StatusCode::Ok);
    /// });
    /// ```
    fn post(&mut self, controller: Controller) -> &mut Self {
        self.endpoints.insert(HttpMethod::POST, controller);
        self
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
    /// router.put(|req, res| {
    ///    res.status(StatusCode::Ok);
    /// });
    /// ```
    fn put(&mut self, controller: Controller) -> &mut Self {
        self.endpoints.insert(HttpMethod::PUT, controller);
        self
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
    /// router.delete( |req, res| {
    ///    res.status(StatusCode::Ok);
    /// });
    /// ```
    fn delete(&mut self, controller: Controller) -> &mut Self {
        self.endpoints.insert(HttpMethod::DELETE, controller);
        self
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
    /// router.patch( |req, res| {
    ///    res.status(StatusCode::Ok);
    /// });
    /// ```
    fn patch(&mut self, controller: Controller) -> &mut Self {
        self.endpoints.insert(HttpMethod::PATCH, controller);
        self
    }
}
