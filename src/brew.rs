use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use std::collections::BTreeMap;

use crate::pod::Pod;
use crate::provider::{Detected, Provider};

const ANTHROPIC_VERSION: &str = "2023-06-01";
const MAX_TOKENS: u32 = 2048;

pub fn interpolate(template: &str, values: &BTreeMap<String, String>) -> String {
    let mut out = template.to_string();
    for (k, v) in values {
        out = out.replace(&format!("{{{{{k}}}}}"), v);
    }
    out
}

pub fn brew(
    pod: &Pod,
    gathered: &BTreeMap<String, String>,
    detected: &Detected,
    model: &str,
) -> Result<Value> {
    let user_prompt = interpolate(&pod.prompt.user, gathered);
    let system = pod.prompt.system.as_str();
    let schema = &pod.output.schema;
    let key = detected.api_key.as_str();
    let endpoint = detected.info.endpoint;

    match detected.info.provider {
        Provider::Anthropic => brew_anthropic(key, endpoint, model, system, &user_prompt, schema),
        Provider::Google => brew_google(key, endpoint, model, system, &user_prompt, schema),
        Provider::Mistral => brew_mistral(key, endpoint, model, system, &user_prompt, schema),
        Provider::Cohere => brew_cohere(key, endpoint, model, system, &user_prompt, schema),
        Provider::OpenAI
        | Provider::Groq
        | Provider::DeepSeek
        | Provider::Xai
        | Provider::OpenRouter => {
            brew_openai_compat(key, endpoint, model, system, &user_prompt, schema)
        }
    }
}

fn json_schema_object(schema: &BTreeMap<String, String>) -> Value {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();
    for (field, ty) in schema {
        properties.insert(field.clone(), json!({ "type": ty }));
        required.push(field.clone());
    }
    json!({
        "type": "object",
        "properties": properties,
        "required": required
    })
}

fn brew_anthropic(
    api_key: &str,
    endpoint: &str,
    model: &str,
    system: &str,
    user: &str,
    schema: &BTreeMap<String, String>,
) -> Result<Value> {
    let tool = json!({
        "name": "emit_result",
        "description": "Emit the structured result for this kcup.",
        "input_schema": json_schema_object(schema)
    });

    let body = json!({
        "model": model,
        "max_tokens": MAX_TOKENS,
        "system": system,
        "tools": [tool],
        "tool_choice": { "type": "tool", "name": "emit_result" },
        "messages": [{ "role": "user", "content": user }]
    });

    let text = post_json(
        endpoint,
        &[
            ("x-api-key", api_key),
            ("anthropic-version", ANTHROPIC_VERSION),
            ("content-type", "application/json"),
        ],
        &body,
        "Anthropic",
    )?;

    let parsed: Value = serde_json::from_str(&text)
        .context("parsing Anthropic response as JSON")?;
    let content = parsed
        .get("content")
        .and_then(|c| c.as_array())
        .context("Anthropic response missing `content` array")?;
    for block in content {
        if block.get("type").and_then(|t| t.as_str()) == Some("tool_use")
            && let Some(input) = block.get("input")
        {
            return Ok(input.clone());
        }
    }
    bail!("Anthropic returned no tool_use block: {text}")
}

fn brew_openai_compat(
    api_key: &str,
    endpoint: &str,
    model: &str,
    system: &str,
    user: &str,
    schema: &BTreeMap<String, String>,
) -> Result<Value> {
    let mut schema_obj = json_schema_object(schema);
    if let Some(map) = schema_obj.as_object_mut() {
        map.insert("additionalProperties".to_string(), json!(false));
    }

    let body = json!({
        "model": model,
        "messages": [
            { "role": "system", "content": system },
            { "role": "user", "content": user }
        ],
        "response_format": {
            "type": "json_schema",
            "json_schema": {
                "name": "emit_result",
                "strict": true,
                "schema": schema_obj
            }
        }
    });

    let bearer = format!("Bearer {api_key}");
    let text = post_json(
        endpoint,
        &[
            ("authorization", bearer.as_str()),
            ("content-type", "application/json"),
        ],
        &body,
        "OpenAI-compatible",
    )?;

    let parsed: Value = serde_json::from_str(&text)
        .context("parsing OpenAI-compatible response as JSON")?;
    let content = parsed
        .pointer("/choices/0/message/content")
        .and_then(|v| v.as_str())
        .with_context(|| format!("missing /choices/0/message/content in: {text}"))?;
    let result: Value = serde_json::from_str(content)
        .context("model output was not valid JSON")?;
    Ok(result)
}

fn brew_google(
    api_key: &str,
    endpoint_base: &str,
    model: &str,
    system: &str,
    user: &str,
    schema: &BTreeMap<String, String>,
) -> Result<Value> {
    let url = format!("{endpoint_base}/{model}:generateContent?key={api_key}");
    let body = json!({
        "systemInstruction": { "parts": [{ "text": system }] },
        "contents": [{ "role": "user", "parts": [{ "text": user }] }],
        "generationConfig": {
            "responseMimeType": "application/json",
            "responseSchema": json_schema_object(schema)
        }
    });

    let text = post_json(
        &url,
        &[("content-type", "application/json")],
        &body,
        "Google",
    )?;

    let parsed: Value =
        serde_json::from_str(&text).context("parsing Google response as JSON")?;
    let txt = parsed
        .pointer("/candidates/0/content/parts/0/text")
        .and_then(|v| v.as_str())
        .with_context(|| format!("missing /candidates/0/content/parts/0/text in: {text}"))?;
    let result: Value =
        serde_json::from_str(txt).context("Gemini output was not valid JSON")?;
    Ok(result)
}

fn brew_mistral(
    api_key: &str,
    endpoint: &str,
    model: &str,
    system: &str,
    user: &str,
    schema: &BTreeMap<String, String>,
) -> Result<Value> {
    let schema_str = serde_json::to_string_pretty(&json_schema_object(schema))
        .unwrap_or_else(|_| "{}".to_string());
    let augmented_system = format!(
        "{system}\n\nReturn ONLY a JSON object matching this JSON Schema (no prose, no code fences):\n{schema_str}"
    );

    let body = json!({
        "model": model,
        "messages": [
            { "role": "system", "content": augmented_system },
            { "role": "user", "content": user }
        ],
        "response_format": { "type": "json_object" }
    });

    let bearer = format!("Bearer {api_key}");
    let text = post_json(
        endpoint,
        &[
            ("authorization", bearer.as_str()),
            ("content-type", "application/json"),
        ],
        &body,
        "Mistral",
    )?;

    let parsed: Value =
        serde_json::from_str(&text).context("parsing Mistral response as JSON")?;
    let content = parsed
        .pointer("/choices/0/message/content")
        .and_then(|v| v.as_str())
        .with_context(|| format!("missing /choices/0/message/content in: {text}"))?;
    let result: Value =
        serde_json::from_str(content).context("Mistral output was not valid JSON")?;
    Ok(result)
}

fn brew_cohere(
    api_key: &str,
    endpoint: &str,
    model: &str,
    system: &str,
    user: &str,
    schema: &BTreeMap<String, String>,
) -> Result<Value> {
    let body = json!({
        "model": model,
        "messages": [
            { "role": "system", "content": system },
            { "role": "user", "content": user }
        ],
        "response_format": {
            "type": "json_object",
            "json_schema": json_schema_object(schema)
        }
    });

    let bearer = format!("Bearer {api_key}");
    let text = post_json(
        endpoint,
        &[
            ("authorization", bearer.as_str()),
            ("content-type", "application/json"),
        ],
        &body,
        "Cohere",
    )?;

    let parsed: Value =
        serde_json::from_str(&text).context("parsing Cohere response as JSON")?;
    let txt = parsed
        .pointer("/message/content/0/text")
        .and_then(|v| v.as_str())
        .with_context(|| format!("missing /message/content/0/text in: {text}"))?;
    let result: Value =
        serde_json::from_str(txt).context("Cohere output was not valid JSON")?;
    Ok(result)
}

fn post_json(
    url: &str,
    headers: &[(&str, &str)],
    body: &Value,
    label: &str,
) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let mut req = client.post(url);
    for (k, v) in headers {
        req = req.header(*k, *v);
    }
    let resp = req
        .json(body)
        .send()
        .with_context(|| format!("sending request to {label}"))?;
    let status = resp.status();
    let text = resp
        .text()
        .with_context(|| format!("reading {label} response body"))?;
    if !status.is_success() {
        bail!("{label} API error ({status}): {text}");
    }
    Ok(text)
}
