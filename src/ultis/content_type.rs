/// Content type
#[non_exhaustive]
#[derive(Debug)]
pub enum ContentType {
    /// application/json
    Json,
    /// text/html
    Html,
    /// text/plain
    Txt,
    /// text/xml
    Xml,
    /// application/javascript
    Js,
    /// Other Content-Type
    Other(String),
}

impl ContentType {
    /// Create Content-Type from mime str
    #[inline]
    #[must_use]
    pub fn get(content_type: &str) -> Self {
        match content_type {
            "application/json" => Self::Json,
            "application/javascript" => Self::Js,
            "text/html" => Self::Html,
            "text/xml" => Self::Xml,
            "text/plain" => Self::Txt,
            other => Self::Other(other.to_owned()),
        }
    }

    /// Get extension file from Content-Type
    #[inline]
    #[must_use]
    pub const fn get_extension(&self) -> &str {
        match self {
            Self::Json => "json",
            Self::Html => "html",
            Self::Txt => "txt",
            Self::Js => "js",
            Self::Xml => "xml",
            Self::Other(_) => "Unknow",
        }
    }
}
