use anyhow::{Result, bail};
use colored::Colorize;
use serde_json::Value;
use std::collections::BTreeMap;

pub fn validate(value: &Value, schema: &BTreeMap<String, String>) -> Result<()> {
    let obj = value
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("model output is not a JSON object"))?;
    for (field, ty) in schema {
        let v = obj
            .get(field)
            .ok_or_else(|| anyhow::anyhow!("missing field `{field}` in model output"))?;
        let ok = match ty.as_str() {
            "string" => v.is_string(),
            "number" => v.is_number(),
            "integer" => v.is_i64() || v.is_u64(),
            "boolean" => v.is_boolean(),
            "array" => v.is_array(),
            "object" => v.is_object(),
            _ => true,
        };
        if !ok {
            bail!("field `{field}` does not match declared type `{ty}`");
        }
    }
    Ok(())
}

pub fn pretty_print(pod_name: &str, value: &Value) {
    println!("{} {}", "▣".cyan().bold(), pod_name.cyan().bold());
    if let Some(obj) = value.as_object() {
        for (k, v) in obj {
            let rendered = match v {
                Value::String(s) => s.clone(),
                _ => serde_json::to_string_pretty(v).unwrap_or_else(|_| v.to_string()),
            };
            println!("  {} {}", format!("{k}:").yellow().bold(), rendered);
        }
    } else {
        println!("{}", value);
    }
}
