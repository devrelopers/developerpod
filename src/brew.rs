use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use std::collections::BTreeMap;

use crate::pod::Pod;

const ANTHROPIC_URL: &str = "https://api.anthropic.com/v1/messages";
const MODEL: &str = "claude-sonnet-4-6";
const ANTHROPIC_VERSION: &str = "2023-06-01";

pub fn interpolate(template: &str, values: &BTreeMap<String, String>) -> String {
    let mut out = template.to_string();
    for (k, v) in values {
        out = out.replace(&format!("{{{{{k}}}}}"), v);
    }
    out
}

pub fn brew(pod: &Pod, gathered: &BTreeMap<String, String>) -> Result<Value> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .context("ANTHROPIC_API_KEY environment variable is not set")?;

    let user_prompt = interpolate(&pod.prompt.user, gathered);
    let tool = build_tool_for_schema(&pod.output.schema);

    let body = json!({
        "model": MODEL,
        "max_tokens": 2048,
        "system": pod.prompt.system,
        "tools": [tool],
        "tool_choice": { "type": "tool", "name": "emit_result" },
        "messages": [
            { "role": "user", "content": user_prompt }
        ]
    });

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(ANTHROPIC_URL)
        .header("x-api-key", api_key)
        .header("anthropic-version", ANTHROPIC_VERSION)
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .context("sending request to Anthropic API")?;

    let status = resp.status();
    let text = resp.text().context("reading Anthropic API response body")?;
    if !status.is_success() {
        bail!("Anthropic API error ({status}): {text}");
    }

    let parsed: Value =
        serde_json::from_str(&text).context("parsing Anthropic API response as JSON")?;

    let content = parsed
        .get("content")
        .and_then(|c| c.as_array())
        .context("response missing `content` array")?;

    for block in content {
        if block.get("type").and_then(|t| t.as_str()) == Some("tool_use")
            && let Some(input) = block.get("input")
        {
            return Ok(input.clone());
        }
    }

    bail!("model returned no tool_use block: {text}")
}

fn build_tool_for_schema(schema: &BTreeMap<String, String>) -> Value {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();
    for (field, ty) in schema {
        properties.insert(field.clone(), json!({ "type": ty }));
        required.push(field.clone());
    }
    json!({
        "name": "emit_result",
        "description": "Emit the structured result for this kcup.",
        "input_schema": {
            "type": "object",
            "properties": properties,
            "required": required
        }
    })
}
