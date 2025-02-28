use owo_colors::OwoColorize;

/// Define log level colors
pub enum LogLevel {
    /// When success
    Success,
    /// When show infomation
    Info,
    /// When show error
    Error,
}

impl LogLevel {
    /// Return color and log level as String
    pub fn fmt(&self) -> String {
        match self {
            Self::Success => "[SUCCESS]".green().bold().to_string(),
            Self::Info => "[INFO]".cyan().bold().to_string(),
            Self::Error => "[ERR]".red().bold().to_string(),
        }
    }
}

///  Test color display implement.
#[test]
fn fmt_color_test() {
    use chrono::Local;
    use std::io::{self, Write};

    let time_now = Local::now().format("[%Y-%m-%d %H:%M:%S]");

    let mut stdout = io::stdout();

    let error_color = LogLevel::Error.fmt();
    let success_color = LogLevel::Success.fmt();
    let info_color = LogLevel::Info.fmt();

    writeln!(stdout, "Error color: {error_color}").unwrap();
    writeln!(stdout, "Success color: {success_color}").unwrap();
    writeln!(stdout, "Info color {info_color}").unwrap();
    writeln!(stdout, "Timestamp now: {time_now}").unwrap();
}
