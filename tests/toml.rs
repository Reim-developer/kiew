use std::io::stdout;
use std::io::Write;

use anyhow::anyhow;
use serde_json::Value as JsonValue;
use toml::Value;

#[test]
fn toml_test() -> Result<(), anyhow::Error> {
    let toml_str = include_str!("test.toml");
    let config: Value = toml::from_str(toml_str)?;

    let raw_header = config["headers"]["body"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing HEADER in body"))?;

    let header: JsonValue = serde_json::from_str(raw_header)?;

    let mut out = stdout();
    writeln!(out, "{header}")?;

    Ok(())
}
