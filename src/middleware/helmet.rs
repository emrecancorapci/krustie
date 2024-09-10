//! Helmet is a collection of 14 smaller middleware functions that set HTTP headers to secure your app. It's not a silver bullet, but it can help!

use configs::{CrossOriginEmbedderPolicy, CrossOriginOpenerPolicy, CrossOriginResourcePolicy, ReferrerPolicy, XFrameOptions, XPermittedCrossDomainPolicy};

use crate::{ Request, Response };

use super::Middleware;

pub mod configs;

#[derive(Clone)]
struct Helmet;

impl Helmet {
    /// The Origin-Agent-Cluster header provides a mechanism to allow web applications to isolate their origins from other processes.
    ///
    /// # Default
    ///
    /// ```
    /// Origin-Agent-Cluster: ?1
    /// ```
    ///
    /// Read more about it in [the spec](https://whatpr.org/html/6214/origin.html#origin-keyed-agent-clusters).
    fn origin_agent_cluster(&self) -> impl Fn(&Request, &mut Response) {
        |_: &Request, response: &mut Response| {
            response.set_header("Origin-Agent-Cluster", "?1");
        }
    }

    /// The Strict-Transport-Security header tells browsers to prefer HTTPS instead of insecure HTTP.
    ///
    /// # Default
    ///
    /// ```
    /// Strict-Transport-Security: max-age=15552000; includeSubDomains
    /// ```
    ///
    /// Read more about it in [the spec](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Strict-Transport-Security).
    ///
    /// # Arguments
    ///
    /// * `options`: `StrictTransportSecurityOptions` - The options for the Strict-Transport-Security header.
    ///
    /// ## StrictTransportSecurityOptions
    ///
    /// * `max_age`: `u128` - The number of seconds browsers should remember to prefer HTTPS. If passed a non-integer, the value is rounded down. It defaults to `15552000`, which is 180 days.
    /// * `include_sub_domains`: `bool` - Dictates whether to include the includeSubDomains directive, which makes this policy extend to subdomains. It defaults to `true`.
    /// * `directives`: `bool` -  If true, it adds the preload directive, expressing intent to add your HSTS policy to browsers. See the [Preloading Strict Transport Security](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Strict-Transport-Security#preloading_strict_transport_security) section on MDN for more. It defaults to `false`.
    fn strict_transport_security(
        &self,
        options: StrictTransportSecuirtyOptions
    ) -> impl Fn(&Request, &mut Response) {
        let header = format!(
            "max-age={}{}{}",
            options.max_age,
            if options.include_sub_domains {
                "; includeSubDomains;"
            } else {
                ""
            },
            if options.directives {
                "; preload"
            } else {
                ""
            }
        );

        move |_: &Request, response: &mut Response| {
            response.set_header("Strict-Transport-Security", header.as_str());
        }
    }

    fn x_content_type_options(&self) -> impl Fn(&Request, &mut Response) {
        |_: &Request, response: &mut Response| {
            response.set_header("X-Content-Type-Options", "nosniff");
        }
    }

    fn x_dns_prefetch_control(&self) -> impl Fn(&Request, &mut Response) {
        |_: &Request, response: &mut Response| {
            response.set_header("X-DNS-Prefetch-Control", "off");
        }
    }

    fn x_download_options(&self) -> impl Fn(&Request, &mut Response) {
        |_: &Request, response: &mut Response| {
            response.set_header("X-Download-Options", "noopen");
        }
    }

    fn x_frame_options(&self, option: XFrameOptions) -> impl Fn(&Request, &mut Response) {
        let policy_string = match option {
            XFrameOptions::SameOrigin => "SAMEORIGIN",
            XFrameOptions::Deny => "DENY",
        };

        move |_: &Request, response: &mut Response| {
            response.set_header("X-Frame-Options", policy_string);
        }
    }

    fn x_powered_by(&self) -> impl Fn(&Request, &mut Response) {
        |_: &Request, response: &mut Response| {
            response.remove_header("X-Powered-By");
        }
    }
    /// The Cross-Origin-Embedder-Policy header helps control what resources can be loaded cross-origin.
    ///
    /// *This header is **not** set by default.*
    ///
    /// See [MDN’s article](https://developer.cdn.mozilla.net/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy) on this header for more.
    fn cross_origin_embedder_policy(
        &self,
        policy: CrossOriginEmbedderPolicy
    ) -> impl Fn(&Request, &mut Response) {
        let policy_string = match policy {
            CrossOriginEmbedderPolicy::RequireCorp => "require-corp",
            CrossOriginEmbedderPolicy::Credentialless => "credentialles",
            CrossOriginEmbedderPolicy::UnsafeNone => "unsafe-none",
        };

        let policy = &policy_string;

        |_: &Request, response: &mut Response| {
            response.set_header("Cross-Origin-Embedder-Policy", policy);
        }
    }

    /// The Cross-Origin-Opener-Policy header helps process-isolate your page.
    ///
    /// # Default
    ///
    /// ```
    /// Cross-Origin-Opener-Policy: same-origin
    /// ```
    ///
    /// See [MDN’s article](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy) on this header for more.
    fn cross_origin_opener_policy(
        &self,
        policy: CrossOriginOpenerPolicy
    ) -> impl Fn(&Request, &mut Response) {
        let policy_string = match policy {
            CrossOriginOpenerPolicy::SameOrigin => "same-origin",
            CrossOriginOpenerPolicy::SameOriginAllowPopups => "same-origin-allow-popups",
            CrossOriginOpenerPolicy::UnsafeNone => "unsafe-none",
        };

        let policy = &policy_string;

        |_: &Request, response: &mut Response| {
            response.set_header("Cross-Origin-Opener-Policy", policy);
        }
    }

    /// The Cross-Origin-Resource-Policy header blocks others from loading your resources cross-origin in some cases.
    ///
    /// # Default
    ///
    /// ```
    /// Cross-Origin-Resource-Policy: same-origin
    /// ```
    ///
    /// See [MDN’s article](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Resource-Policy) on this header for more.
    fn cross_origin_resource_policy(
        &self,
        policy: CrossOriginResourcePolicy
    ) -> impl Fn(&Request, &mut Response) {
        let policy_string = match policy {
            CrossOriginResourcePolicy::SameOrigin => "same-origin",
            CrossOriginResourcePolicy::SameSite => "same-site",
            CrossOriginResourcePolicy::CrossOrigin => "cross-origin",
        };

        let policy = &policy_string;

        |_: &Request, response: &mut Response| {
            response.set_header("Cross-Origin-Resource-Policy", policy);
        }
    }

    /// The Referrer-Policy header which controls what information is set in the Referer request header.
    ///
    /// # Default
    ///
    /// ```
    /// Referrer-Policy: no-referrer
    /// ```
    ///
    /// See [MDN’s article](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referrer-Policy) on this header for more.
    fn referrer_policy(&self, policies: Vec<ReferrerPolicy>) -> impl Fn(&Request, &mut Response) {
        let policy_string = policies
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(",");

        move |_: &Request, response: &mut Response| {
            response.set_header("Referrer-Policy", policy_string.as_str());
        }
    }

    fn x_permitted_cross_domain_policies(
        &self,
        policy: XPermittedCrossDomainPolicy
    ) -> impl Fn(&Request, &mut Response) {
        let header = match policy {
            XPermittedCrossDomainPolicy::All => "all",
            XPermittedCrossDomainPolicy::ByContentType => "by-content-type",
            XPermittedCrossDomainPolicy::MasterOnly => "master-only",
            XPermittedCrossDomainPolicy::None => "none",
        };

        |_: &Request, response: &mut Response| {
            response.set_header("X-Permitted-Cross-Domain-Policies", header);
        }
    }
}

impl Default for Helmet {
    fn default() -> Self {
        Self {  }
    }
}

impl Middleware for Helmet {
    fn middleware(&mut self, request: &Request, response: &mut Response) -> crate::server::route_handler::HandlerResult {
        todo!()
    }
}

/// Configuration options for the Strict-Transport-Security header.
pub struct StrictTransportSecuirtyOptions {
    max_age: u128,
    include_sub_domains: bool,
    directives: bool,
}

impl Default for StrictTransportSecuirtyOptions {
    /// Returns the default configuration for the Strict-Transport-Security header.
    ///
    /// # Default
    ///
    /// ```
    /// StrictTransportSecuirtyOptions {
    ///   max_age: 15552000,
    ///   include_sub_domains: true,
    ///   directives: false,
    /// }
    fn default() -> Self {
        Self {
            max_age: 15552000,
            include_sub_domains: true,
            directives: false,
        }
    }
}
