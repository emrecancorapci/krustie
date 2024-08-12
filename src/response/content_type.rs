//! This module contains the content type enum for response content type

use std::fmt::{Debug, Display};

/// Content type enum for response content type
#[derive(Debug)]
pub enum ContentType {
    // Text
    /// Represents the `text/plain` content type
    Text,
    /// Represents the `text/html` content type
    Html,
    /// Represents the `text/css` content type
    Css,
    /// Represents the `text/csv` content type
    Csv,
    /// Represents the `text/javascript` content type
    Javascript,
    // Application
    /// Represents the `application/json` content type
    Json,
    /// Represents the `application/gzip` content type
    Gzip,
    /// Represents the `application/xml` content type
    Xml,
    /// Represents the `application/pdf` content type
    Pdf,
    /// Represents the `application/zip` content type
    Zip,
    // Image
    /// Represents the `image/png` content type
    Png,
    /// Represents the `image/jpeg` content type
    Jpeg,
    /// Represents the `image/x-icon` content type
    Icon,
    /// Represents the `image/gif` content type
    Gif,
    /// Represents the `image/bmp` content type
    Bmp,
    /// Represents the `image/webp` content type
    Webp,
    /// Represents the `image/tiff` content type
    Tiff,
    // Audio
    /// Represents the `audio/mp3` content type
    Mp3,
    /// Represents the `audio/wav` content type
    Wav,
    /// Represents the `audio/ogg` content type
    Ogg,
    /// Represents the `audio/midi` content type
    Midi,
    /// Represents the `audio/webm` content type
    AudioWebm,
    // Video
    /// Represents the `video/mp4` content type
    Mp4,
    /// Represents the `video/webm` content type
    Webm,
    /// Represents the `video/ogg` content type
    OggVideo,
    /// Represents the `video/mpeg` content type
    Mpeg,
    // Font
    /// Represents the `font/woff` content type
    Woff,
    /// Represents the `font/woff2` content type
    Woff2,
    /// Represents the `font/ttf` content type
    Ttf,
    /// Represents the `font/otf` content type
    Otf,
    /// Represents any other content type
    Other(String),
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Text => write!(f, "text/plain"),
            ContentType::Html => write!(f, "text/html"),
            ContentType::Css => write!(f, "text/css"),
            ContentType::Csv => write!(f, "text/csv"),
            ContentType::Javascript => write!(f, "text/javascript"),
            ContentType::Json => write!(f, "application/json"),
            ContentType::Xml => write!(f, "application/xml"),
            ContentType::Pdf => write!(f, "application/pdf"),
            ContentType::Gzip => write!(f, "application/gzip"),
            ContentType::Zip => write!(f, "application/zip"),
            ContentType::Png => write!(f, "image/png"),
            ContentType::Jpeg => write!(f, "image/jpeg"),
            ContentType::Icon => write!(f, "image/x-icon"),
            ContentType::Gif => write!(f, "image/gif"),
            ContentType::Bmp => write!(f, "image/bmp"),
            ContentType::Webp => write!(f, "image/webp"),
            ContentType::Tiff => write!(f, "image/tiff"),
            ContentType::Mp3 => write!(f, "audio/mp3"),
            ContentType::Wav => write!(f, "audio/wav"),
            ContentType::Ogg => write!(f, "audio/ogg"),
            ContentType::Midi => write!(f, "audio/midi"),
            ContentType::AudioWebm => write!(f, "audio/webm"),
            ContentType::Mp4 => write!(f, "video/mp4"),
            ContentType::Mpeg => write!(f, "video/mpeg"),
            ContentType::Webm => write!(f, "video/webm"),
            ContentType::OggVideo => write!(f, "video/ogg"),
            ContentType::Woff => write!(f, "font/woff"),
            ContentType::Woff2 => write!(f, "font/woff2"),
            ContentType::Ttf => write!(f, "font/ttf"),
            ContentType::Otf => write!(f, "font/otf"),
            ContentType::Other(content_type) => write!(f, "{}", content_type),
        }
    }
}

impl TryFrom<&str> for ContentType {
    type Error = String;
    fn try_from(ext: &str) -> Result<ContentType, String> {
        match ext {
            "txt" => Ok(ContentType::Text),
            "html" => Ok(ContentType::Html),
            "css" => Ok(ContentType::Css),
            "csv" => Ok(ContentType::Csv),
            "js" => Ok(ContentType::Javascript),
            "json" => Ok(ContentType::Json),
            "xml" => Ok(ContentType::Xml),
            "pdf" => Ok(ContentType::Pdf),
            "gzip" => Ok(ContentType::Gzip),
            "zip" => Ok(ContentType::Zip),
            "png" => Ok(ContentType::Png),
            "jpg" => Ok(ContentType::Jpeg),
            "jpeg" => Ok(ContentType::Jpeg),
            "ico" => Ok(ContentType::Icon),
            "gif" => Ok(ContentType::Gif),
            "bmp" => Ok(ContentType::Bmp),
            "webp" => Ok(ContentType::Webp),
            "tiff" => Ok(ContentType::Tiff),
            "mp3" => Ok(ContentType::Mp3),
            "wav" => Ok(ContentType::Wav),
            "midi" => Ok(ContentType::Midi),
            "mp4" => Ok(ContentType::Mp4),
            "mpeg" => Ok(ContentType::Mpeg),
            "webm" => Ok(ContentType::Webm),
            "ogg" => Ok(ContentType::OggVideo),
            "woff" => Ok(ContentType::Woff),
            "woff2" => Ok(ContentType::Woff2),
            "ttf" => Ok(ContentType::Ttf),
            "otf" => Ok(ContentType::Otf),
            other => Err(format!("Unknown content type for extension: {}", other)),
        }
    }
}
