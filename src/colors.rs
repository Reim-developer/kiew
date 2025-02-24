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
            Self::Success => "[SUCCESS]".bright_white().on_bright_green().to_string(),
            Self::Info => "[INFO]".bright_white().on_cyan().to_string(),
            Self::Error => "[ERR]".bright_white().on_bright_red().to_string(),
        }
    }
}
/// For log stdout
#[macro_export]
macro_rules! log_stdout {
    ($($arg:tt)*) => {{
        use chrono::Local;
        let time_now = Local::now().format("[%Y-%m-%d %H:%M:%S]");

        println!("{} {}", time_now, format!($($arg)*));
    }};
}

/// For log stderr
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {{
        use chrono::Local;
        let time_now = Local::now().format("[%Y-%m-%d %H:%M:%S]");

        eprintln!("{} {}", time_now, format!($($arg)*));
    }};
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
