use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;

#[test]
fn read_json() -> Result<(), anyhow::Error> {
    //     let json_source = r#"
    // {
    //     "website_url": [
    //         "",
    //         ""
    //     ],
    //     "headers": {
    //         "key": "value"
    //     }
    // }
    // "#;

    let json_from_file = include_str!("13_37_29.json");

    let json_value: Value = serde_json::from_str(json_from_file)?;

    let headers: HashMap<Cow<str>, Cow<str>> = json_value
        .get("headers")
        .and_then(Value::as_object)
        .map(|body| {
            let mut map = HashMap::with_capacity(body.len());
            for (k, v) in body {
                if let Some(v_str) = v.as_str() {
                    map.insert(Cow::Borrowed(k.as_str()), Cow::Borrowed(v_str));
                }
            }
            map
        })
        .unwrap_or_default();

    let mut out = stdout();
    for (key, value) in headers {
        writeln!(out, "{key}: {value}")?;
    }

    Ok(())
}
