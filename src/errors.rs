/// Public Errors Type for fatal error
pub enum ErrorsType {
    /// When cannot find any HTML element.
    ElementNotFound,
    /// When Parse HTML is failed
    HtmlParseFailed,
    /// When request is failed
    RequestFailed,
}
impl ErrorsType {
    /// Convert Error Type to str
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::ElementNotFound => "ELEMENT_NOT_FOUND",
            Self::HtmlParseFailed => "HTML_PARSE_FAILED",
            Self::RequestFailed => "REQUEST_FAILED",
        }
    }
}
