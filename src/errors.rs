pub enum ErrorsType {
    ElementNotFound,
    RequestFailed,
    HtmlParseFailed,
}

impl ErrorsType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorsType::ElementNotFound => "ELEMENT_NOT_FOUND",
            ErrorsType::RequestFailed => "REQUEST_FAILED",
            ErrorsType::HtmlParseFailed => "HTML_PARSE_FAILED",
        }
    }
}
