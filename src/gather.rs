use anyhow::{Context, Result, bail};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use crate::pod::Gatherer;

pub fn run_all(gatherers: &[Gatherer]) -> Result<BTreeMap<String, String>> {
    let mut out = BTreeMap::new();
    for g in gatherers {
        let value = if let Some(cmd) = &g.shell {
            run_shell(cmd).with_context(|| format!("gatherer '{}' (shell)", g.id))?
        } else if let Some(path) = &g.file {
            match read_file(Path::new(path)) {
                Ok(s) => s,
                Err(e) => {
                    if g.optional {
                        String::new()
                    } else {
                        return Err(e).with_context(|| format!("gatherer '{}' (file)", g.id));
                    }
                }
            }
        } else {
            bail!("gatherer '{}' has no source", g.id);
        };
        out.insert(g.id.clone(), value);
    }
    Ok(out)
}

fn run_shell(cmd: &str) -> Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .with_context(|| format!("spawning shell command: {cmd}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("shell command failed ({}): {}", output.status, stderr.trim());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim_end().to_string())
}

fn read_file(path: &Path) -> Result<String> {
    let s = std::fs::read_to_string(path)
        .with_context(|| format!("reading file {}", path.display()))?;
    Ok(s)
}
