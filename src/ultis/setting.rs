use crate::colors::{LogLevel::Info, LogLevel::Success};
use crate::log_stdout;
use anyhow::{anyhow, Result};
use chrono::Local;
use std::fs;
use std::path::Path;
use std::{fs::File, io::Write};

/// Generator setting with customize setting file name. Default is
/// system timestamp
///
/// # Errors
/// - `File already exists`
/// - `Create file fails`
#[inline]
pub fn generate_setting(file_name: &str) -> Result<(), anyhow::Error> {
    let time_now = Local::now().format("%H_%M_%S");
    let raw_setting = r#"
{   
    "website_url": [
        "",
        ""
    ],
    "headers": {
        "X-Custom-Header": "",
        "Authorization": "",
        "Cache-Control": "",
        "Security-Policies": "",
        "Content-Type":  "",
        "Accept-Language": "",
    }
}
    
"#;
    let file_setting = if file_name.is_empty() {
        format!("{time_now}.json")
    } else {
        let setting = if Path::new(file_name)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
        {
            file_name.to_owned()
        } else {
            format!("{file_name}.json")
        };
        if let Some(dir) = Path::new(&setting).parent() {
            fs::create_dir_all(dir)?;
        }
        setting
    };

    if Path::new(&file_setting).exists() {
        return Err(anyhow!("{file_setting} already exists."));
    }

    let mut file = File::create(&file_setting)?;
    let bytes_written = file.write(raw_setting.as_bytes())?;

    log_stdout!(
        "{} Generated setting file successfully: {} ({} bytes)",
        Success.fmt(),
        file_setting,
        bytes_written
    );

    if file_name.is_empty() {
        log_stdout!(
            "{} To customize your file name, just use -F <YOUR FILE SETTING NAME>",
            Info.fmt()
        );
    }

    Ok(())
}
