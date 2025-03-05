use std::io::stdout;
use std::io::Write;

use anyhow::anyhow;
use toml::Value;

#[test]
fn reader_toml() -> Result<(), anyhow::Error> {
    let toml_source = include_str!("test.toml");
    let toml_str: Value = toml::from_str(toml_source)?;

    let website = match &toml_str["website"]["target"] {
        Value::String(value) => vec![value.as_str()],
        Value::Array(value) => value.iter().filter_map(|value| value.as_str()).collect(),
        _ => {
            return Err(anyhow!(
                "Website target field is not support current value type"
            ))
        }
    };

    let mut stdout = stdout();

    for target in website {
        writeln!(stdout, "{target}")?;
    }

    Ok(())
}
