use crate::types::WorkspaceConfig;
use anyhow::{bail, Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn load_config(path: impl AsRef<Path>) -> Result<WorkspaceConfig> {
    let p = path.as_ref();
    let content = fs::read_to_string(p).with_context(|| format!("read {}", p.display()))?;
    let ext = p
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "yaml" | "yml" => {
            let cfg: WorkspaceConfig = serde_yaml::from_str(&content).context("parse yaml")?;
            Ok(cfg)
        }
        "json" => {
            let cfg: WorkspaceConfig = serde_json::from_str(&content).context("parse json")?;
            Ok(cfg)
        }
        _ => {
            let yaml = serde_yaml::from_str::<WorkspaceConfig>(&content);
            if let Ok(cfg) = yaml {
                return Ok(cfg);
            }
            let json = serde_json::from_str::<WorkspaceConfig>(&content);
            if let Ok(cfg) = json {
                return Ok(cfg);
            }
            bail!("unsupported config format");
        }
    }
}

pub fn validate_config(cfg: &WorkspaceConfig) -> Result<()> {
    let mut names = HashSet::new();
    for s in &cfg.services {
        if !names.insert(&s.name) {
            bail!("duplicate service name {}", s.name);
        }
    }
    Ok(())
}
