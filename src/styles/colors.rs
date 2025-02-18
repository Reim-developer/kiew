use owo_colors::OwoColorize;

pub enum LogLevel {
    Success,
    Warning,
    Error,
}

impl LogLevel {
    pub fn fmt(&self) -> String {
        match self {
            LogLevel::Success => "[SUCCESS]".bright_white().on_bright_green().to_string(),
            LogLevel::Warning => "[WARN]".bright_black().on_bright_yellow().to_string(),
            LogLevel::Error => "[ERR]".bright_white().on_bright_red().to_string(),
        }
    }
}

///  Test color display implement.
#[test]
fn fmt_color_test() {
    use std::io::{self, Write};

    let mut stdout = io::stdout();

    let error_color = LogLevel::Error.fmt();
    let success_color = LogLevel::Success.fmt();
    let warn_color = LogLevel::Warning.fmt();

    writeln!(stdout, "Error color: {}", error_color).unwrap();
    writeln!(stdout, "Success color: {}", success_color).unwrap();
    writeln!(stdout, "Warn color: {}", warn_color).unwrap();
}
