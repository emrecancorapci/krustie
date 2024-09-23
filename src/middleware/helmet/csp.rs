use std::{collections::HashMap, fmt::Display};

#[derive(Eq, PartialEq, Hash)]
pub enum Directive {
    BaseUri,
    ChlidSrc,
    ConnectSrc,
    DefaultSrc,
    FontSrc,
    FormAction,
    FrameAncestors,
    FrameSrc,
    ImgSrc,
    ManifestSrc,
    MediaSrc,
    ObjectSrc,
    ReportTo,
    Sandbox,
    ScriptSrc,
    ScriptSrcElem,
    ScriptSrcAttr,
    StyleSrc,
    StyleSrcElem,
    StyleSrcAttr,
    UpgradeInsecureRequests,
    WorkerSrc,
}

pub enum DirectiveValues {
    None,
    _Self,
    StrictDynamic,
    ReportSample,
    UnsafeInline,
    UnsafeEval,
    UnsafeHashes,
    WasmUnsafeEval,
    Host(String),
    Scheme(String),
    Nonce(String),
    Sha(String),
}

pub struct ContentSecurityPolicy {
    directives: HashMap<Directive, Vec<DirectiveValues>>,
}

impl ContentSecurityPolicy {
    pub fn render(&self) -> String {
        let mut csp = String::new();

        for (directive, values) in self.directives.iter() {
            csp.push_str(&format!("{} ", directive.to_string()));

            for value in values.iter() {
                csp.push_str(&format!("{} ", value.to_string()));
            }

            csp.push_str(";");
        }

        csp
    }

    pub fn set_directive(&mut self, directive: Directive, values: Vec<DirectiveValues>) {
        self.directives.insert(directive, values);
    }
}

impl Default for ContentSecurityPolicy {
    fn default() -> Self {
        let mut directives = HashMap::new();

        directives.insert(Directive::DefaultSrc, vec![DirectiveValues::_Self]);
        directives.insert(Directive::BaseUri, vec![DirectiveValues::_Self]);
        directives.insert(
            Directive::FontSrc,
            vec![
                DirectiveValues::_Self,
                DirectiveValues::Scheme("data".to_string()),
                DirectiveValues::Scheme("https".to_string()),
            ],
        );
        directives.insert(Directive::FormAction, vec![DirectiveValues::_Self]);
        directives.insert(Directive::FrameAncestors, vec![DirectiveValues::_Self]);
        directives.insert(
            Directive::ImgSrc,
            vec![
                DirectiveValues::_Self,
                DirectiveValues::Scheme("data".to_string()),
            ],
        );
        directives.insert(Directive::ObjectSrc, vec![DirectiveValues::None]);
        directives.insert(Directive::ScriptSrc, vec![DirectiveValues::_Self]);
        directives.insert(Directive::ScriptSrcAttr, vec![DirectiveValues::None]);
        directives.insert(
            Directive::StyleSrc,
            vec![
                DirectiveValues::_Self,
                DirectiveValues::Scheme("https".to_string()),
                DirectiveValues::UnsafeInline,
            ],
        );
        directives.insert(Directive::UpgradeInsecureRequests, vec![]);

        Self { directives }
    }
}

impl Display for DirectiveValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string =         match self {
            DirectiveValues::None => "".to_string(),
            DirectiveValues::_Self => "'self'".to_string(),
            DirectiveValues::StrictDynamic => "'strict-dynamic'".to_string(),
            DirectiveValues::ReportSample => "'report-sample'".to_string(),
            DirectiveValues::UnsafeInline => "'unsafe-inline'".to_string(),
            DirectiveValues::UnsafeEval => "'unsafe-eval'".to_string(),
            DirectiveValues::UnsafeHashes => "'unsafe-hashes'".to_string(),
            DirectiveValues::WasmUnsafeEval => "'wasm-unsafe-eval'".to_string(),
            DirectiveValues::Host(host) => host.to_string(),
            DirectiveValues::Scheme(scheme) => format!("{}:", scheme).to_string(),
            DirectiveValues::Nonce(nonce) => format!("'nonce-{}'", nonce),
            DirectiveValues::Sha(sha) => format!("'sha-{}'", sha),
        };

        write!(f, "{}", string)
    }
}

impl Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Directive::BaseUri => "base-uri".to_string(),
            Directive::ChlidSrc => "child-src".to_string(),
            Directive::ConnectSrc => "connect-src".to_string(),
            Directive::DefaultSrc => "default-src".to_string(),
            Directive::FontSrc => "font-src".to_string(),
            Directive::FormAction => "form-action".to_string(),
            Directive::FrameAncestors => "frame-ancestors".to_string(),
            Directive::FrameSrc => "frame-src".to_string(),
            Directive::ImgSrc => "img-src".to_string(),
            Directive::ManifestSrc => "manifest-src".to_string(),
            Directive::MediaSrc => "media-src".to_string(),
            Directive::ObjectSrc => "object-src".to_string(),
            Directive::ReportTo => "report-to".to_string(),
            Directive::Sandbox => "sandbox".to_string(),
            Directive::ScriptSrc => "script-src".to_string(),
            Directive::ScriptSrcElem => "script-src-elem".to_string(),
            Directive::ScriptSrcAttr => "script-src-attr".to_string(),
            Directive::StyleSrc => "style-src".to_string(),
            Directive::StyleSrcElem => "style-src-elem".to_string(),
            Directive::StyleSrcAttr => "style-src-attr".to_string(),
            Directive::UpgradeInsecureRequests => "upgrade-insecure-requests".to_string(),
            Directive::WorkerSrc => "worker-src".to_string(),
        };

        write!(f, "{}", string)
    }
}