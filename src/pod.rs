use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct Pod {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default, rename = "gather")]
    pub gather: Vec<Gatherer>,
    pub prompt: Prompt,
    pub output: Output,
}

#[derive(Debug, Deserialize)]
pub struct Gatherer {
    pub id: String,
    #[serde(default)]
    pub shell: Option<String>,
    #[serde(default)]
    pub file: Option<String>,
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Deserialize)]
pub struct Prompt {
    pub system: String,
    pub user: String,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub schema: BTreeMap<String, String>,
}

pub fn resolve_pod_path(name: &str) -> Result<PathBuf> {
    let direct = PathBuf::from(format!("{name}.kcup.toml"));
    if direct.exists() {
        return Ok(direct);
    }
    let in_examples = PathBuf::from(format!("examples/{name}.kcup.toml"));
    if in_examples.exists() {
        return Ok(in_examples);
    }
    bail!(
        "could not find kcup '{name}'. Looked for ./{name}.kcup.toml and ./examples/{name}.kcup.toml"
    );
}

pub fn load_pod(path: &Path) -> Result<Pod> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("reading kcup file {}", path.display()))?;
    let pod: Pod = toml::from_str(&raw)
        .with_context(|| format!("parsing kcup file {}", path.display()))?;
    for g in &pod.gather {
        if g.shell.is_none() && g.file.is_none() {
            bail!("gatherer '{}' must define either `shell` or `file`", g.id);
        }
        if g.shell.is_some() && g.file.is_some() {
            bail!(
                "gatherer '{}' cannot define both `shell` and `file`",
                g.id
            );
        }
    }
    Ok(pod)
}
