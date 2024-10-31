use std::{
    collections::HashMap,
    net::IpAddr,
    time::{Duration, Instant},
};

use crate::{server::route_handler::HandlerResult, Middleware, StatusCode};

/// A rate limiter middleware
///
/// Limits the number of requests from an IP address based on the token number and token refill time.
#[derive(Clone, Debug)]
pub struct RateLimiter {
    token_number: u16,
    token_refill_duration: Duration,
    requests: HashMap<IpAddr, (Instant, u16)>,
}

impl RateLimiter {
    /// Creates a new instance of RateLimiter
    ///
    /// # Example
    ///
    /// ```rust
    /// use krustie::{ Server, middleware::RateLimiter };
    ///
    /// let mut server = Server::create();
    ///
    /// let rate_limiter = RateLimiter::new(10, 1000);
    ///
    /// server.use_handler(rate_limiter);
    /// ```
    pub fn new(token_number: u16, token_refill_ms: u64) -> Self {
        Self {
            token_number,
            token_refill_duration: Duration::from_millis(token_refill_ms),
            requests: HashMap::new(),
        }
    }

    fn check(&mut self, ip: IpAddr) -> bool {
        let now: Instant = Instant::now();

        let entry = self
            .requests
            .entry(ip)
            .or_insert_with(|| (now, self.token_number));

        if now.duration_since(entry.0) >= self.token_refill_duration {
            // Duration passed, token refreshed
            entry.0 = now;
            entry.1 = self.token_number - 1;
            true
        } else if entry.1 > 0 {
            // Duration hasn't passed, token reduced
            entry.1 -= 1;
            true
        } else {
            // No tokens left
            false
        }
    }
}

impl Middleware for RateLimiter {
    fn middleware(
        &mut self,
        request: &crate::Request,
        response: &mut crate::Response,
    ) -> HandlerResult {
        match Self::check(self, request.get_peer_addr().ip()) {
            true => HandlerResult::Next,
            false => {
                response.status(StatusCode::TooManyRequests);
                return HandlerResult::End;
            }
        }
    }
}
