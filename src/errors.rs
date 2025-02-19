/// Public Errors Type for fatal error
pub enum ErrorsType {
    /// When cannot find any HTML element.
    ElementNotFound,
    /// When parsing HTML is fails
    HtmlParseFailed,
    /// When the request is fails
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
