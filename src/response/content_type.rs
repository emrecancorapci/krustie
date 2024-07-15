//! This module contains the content type enum for response content type

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

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Text => String::from("text/plain"),
            ContentType::Html => String::from("text/html"),
            ContentType::Css => String::from("text/css"),
            ContentType::Csv => String::from("text/csv"),
            ContentType::Javascript => String::from("text/javascript"),
            ContentType::Json => String::from("application/json"),
            ContentType::Xml => String::from("application/xml"),
            ContentType::Pdf => String::from("application/pdf"),
            ContentType::Gzip => String::from("application/gzip"),
            ContentType::Zip => String::from("application/zip"),
            ContentType::Png => String::from("image/png"),
            ContentType::Jpeg => String::from("image/jpeg"),
            ContentType::Icon => String::from("image/x-icon"),
            ContentType::Gif => String::from("image/gif"),
            ContentType::Bmp => String::from("image/bmp"),
            ContentType::Webp => String::from("image/webp"),
            ContentType::Tiff => String::from("image/tiff"),
            ContentType::Mp3 => String::from("audio/mp3"),
            ContentType::Wav => String::from("audio/wav"),
            ContentType::Ogg => String::from("audio/ogg"),
            ContentType::Midi => String::from("audio/midi"),
            ContentType::AudioWebm => String::from("audio/webm"),
            ContentType::Mp4 => String::from("video/mp4"),
            ContentType::Mpeg => String::from("video/mpeg"),
            ContentType::Webm => String::from("video/webm"),
            ContentType::OggVideo => String::from("video/ogg"),
            ContentType::Woff => String::from("font/woff"),
            ContentType::Woff2 => String::from("font/woff2"),
            ContentType::Ttf => String::from("font/ttf"),
            ContentType::Otf => String::from("font/otf"),
            ContentType::Other(content_type) => content_type.clone(),
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
