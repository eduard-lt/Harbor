use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub services: Vec<Service>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub command: String,
    pub cwd: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub depends_on: Option<Vec<String>>,
    pub health_check: Option<HealthCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub kind: HealthCheckKind,
    pub command: Option<String>,
    pub url: Option<String>,
    pub tcp_port: Option<u16>,
    pub timeout_ms: Option<u64>,
    pub retries: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthCheckKind {
    Command,
    Http,
    Tcp,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub extensions: Option<Vec<String>>,
    pub pattern: Option<String>,
    pub min_size_bytes: Option<u64>,
    pub max_size_bytes: Option<u64>,
    pub target_dir: String,
    pub create_symlink: Option<bool>,
    #[serde(default = "default_enabled")]
    pub enabled: Option<bool>,
}

fn default_enabled() -> Option<bool> {
    Some(true)
}
