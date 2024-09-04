use crate::{ Request, Response };
use super::Helmet;

impl Helmet {
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
            response.insert_header("Cross-Origin-Embedder-Policy", policy);
        }
    }

    // Default: SameOrigin
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
            response.insert_header("Cross-Origin-Opener-Policy", policy);
        }
    }

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
            response.insert_header("Cross-Origin-Resource-Policy", policy);
        }
    }

    fn referrer_policy(&self, policies: Vec<ReferrerPolicy>) -> impl Fn(&Request, &mut Response) {
        let policy_string = policies
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<String>>()
        .join(",");

        move |_: &Request, response: &mut Response| {
            response.insert_header("Referrer-Policy", policy_string.as_str());
        }
    }
}

pub enum CrossOriginEmbedderPolicy {
    RequireCorp,
    Credentialless,
    UnsafeNone,
}

pub enum CrossOriginOpenerPolicy {
    SameOrigin,
    SameOriginAllowPopups,
    UnsafeNone,
}

pub enum CrossOriginResourcePolicy {
    SameOrigin,
    SameSite,
    CrossOrigin,
}

pub enum ReferrerPolicy {
    NoReferrer,
    NoReferrerWhenDowngrade,
    SameOrigin,
    Origin,
    StrictOrigin,
    OriginWhenCrossOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

impl ToString for &ReferrerPolicy {
    fn to_string(&self) -> String {
        (
            match self {
                ReferrerPolicy::NoReferrer => "no-referrer",
                ReferrerPolicy::NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
                ReferrerPolicy::SameOrigin => "same-origin",
                ReferrerPolicy::Origin => "origin",
                ReferrerPolicy::StrictOrigin => "strict-origin",
                ReferrerPolicy::OriginWhenCrossOrigin => "origin-when-cross-origin",
                ReferrerPolicy::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
                ReferrerPolicy::UnsafeUrl => "unsafe-url",
            }
        ).to_string()
    }
}
