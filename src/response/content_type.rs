pub enum ContentType {
    // Text
    Text,
    Html,
    Css,
    Csv,
    Javascript,
    // Application
    Json,
    Gzip,
    Xml,
    Pdf,
    Zip,
    // Image
    Png,
    Jpeg,
    Icon,
    Gif,
    Bmp,
    Webp,
    Tiff,
    // Audio
    Mp3,
    Wav,
    Ogg,
    Midi,
    AudioWebm,
    // Video
    Mp4,
    Webm,
    OggVideo,
    Mpeg,
    // Font
    Woff,
    Woff2,
    Ttf,
    Otf,
    // Other
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

impl From<&str> for ContentType {
    fn from(ext: &str) -> ContentType {
        match ext {
            "txt" => ContentType::Text,
            "html" => ContentType::Html,
            "css" => ContentType::Css,
            "csv" => ContentType::Csv,
            "js" => ContentType::Javascript,
            "json" => ContentType::Json,
            "xml" => ContentType::Xml,
            "pdf" => ContentType::Pdf,
            "gzip" => ContentType::Gzip,
            "zip" => ContentType::Zip,
            "png" => ContentType::Png,
            "jpg" => ContentType::Jpeg,
            "jpeg" => ContentType::Jpeg,
            "ico" => ContentType::Icon,
            "gif" => ContentType::Gif,
            "bmp" => ContentType::Bmp,
            "webp" => ContentType::Webp,
            "tiff" => ContentType::Tiff,
            "mp3" => ContentType::Mp3,
            "wav" => ContentType::Wav,
            "midi" => ContentType::Midi,
            "mp4" => ContentType::Mp4,
            "mpeg" => ContentType::Mpeg,
            "webm" => ContentType::Webm,
            "ogg" => ContentType::OggVideo,
            "woff" => ContentType::Woff,
            "woff2" => ContentType::Woff2,
            "ttf" => ContentType::Ttf,
            "otf" => ContentType::Otf,
            other => ContentType::Other(other.to_string()),
        }
    }
}
