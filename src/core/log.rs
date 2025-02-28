/// For log stdout
#[macro_export]
macro_rules! log_stdout {
    ($($arg:tt)*) => {{
        let time_now = chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]");

        println!("{} {}", time_now, format!($($arg)*));
    }};
}

/// For log stderr
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {{
        let time_now = chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]");

        eprintln!("{} {}", time_now, format!($($arg)*));
    }};
}
