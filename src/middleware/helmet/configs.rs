pub enum XFrameOptions {
    SameOrigin,
    Deny,
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

pub enum XPermittedCrossDomainPolicy {
    All,
    ByContentType,
    MasterOnly,
    None,
}
