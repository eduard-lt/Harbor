use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::thread;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadsConfig {
    pub download_dir: String,
    pub rules: Vec<Rule>,
    pub min_age_secs: Option<u64>,
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
}

pub fn load_downloads_config(path: impl AsRef<Path>) -> Result<DownloadsConfig> {
    let p = path.as_ref();
    let content = fs::read_to_string(p).with_context(|| format!("read {}", p.display()))?;
    let mut cfg: DownloadsConfig =
        serde_yaml::from_str(&content).context("parse downloads yaml")?;
    cfg.download_dir = expand_env(&cfg.download_dir);
    for r in cfg.rules.iter_mut() {
        r.target_dir = expand_env(&r.target_dir);
    }
    Ok(cfg)
}

fn is_partial(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".crdownload")
        || lower.ends_with(".part")
        || lower.ends_with(".tmp")
        || lower.ends_with(".download")
}

fn matches_rule(path: &Path, meta: &fs::Metadata, rule: &Rule) -> bool {
    if let Some(exts) = &rule.extensions {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .unwrap_or_default();
        if !exts.iter().any(|x| x.to_ascii_lowercase() == ext) {
            return false;
        }
    }
    if let Some(pat) = &rule.pattern {
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if let Ok(re) = Regex::new(pat) {
                if !re.is_match(name) {
                    return false;
                }
            }
        }
    }
    let size: u64 = meta.len();
    if let Some(min) = rule.min_size_bytes {
        if size < min {
            return false;
        }
    }
    if let Some(max) = rule.max_size_bytes {
        if size > max {
            return false;
        }
    }
    true
}

fn ensure_dir(dir: &Path) -> Result<()> {
    fs::create_dir_all(dir).with_context(|| format!("create {}", dir.display()))?;
    Ok(())
}

fn unique_target(target: &Path) -> PathBuf {
    if !target.exists() {
        return target.to_path_buf();
    }
    let mut i = 1u32;
    loop {
        let mut p = target.to_path_buf();
        let stem = target
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("file");
        let ext = target.extension().and_then(|e| e.to_str()).unwrap_or("");
        let name = if ext.is_empty() {
            format!("{} ({})", stem, i)
        } else {
            format!("{} ({}).{}", stem, i, ext)
        };
        p.set_file_name(name);
        if !p.exists() {
            return p;
        }
        i += 1;
    }
}

pub fn organize_once(cfg: &DownloadsConfig) -> Result<Vec<(PathBuf, PathBuf, String, Option<String>)>> {
    let base = PathBuf::from(&cfg.download_dir);
    let min_age = Duration::from_secs(cfg.min_age_secs.unwrap_or(5));
    let mut actions = Vec::new();
    for entry in fs::read_dir(&base).with_context(|| format!("list {}", base.display()))? {
        let entry = entry?;
        let path = entry.path();
        let meta = match fs::symlink_metadata(&path) {
            Ok(m) => m,
            Err(_) => continue,
        };
        if meta.file_type().is_symlink() || !meta.is_file() {
            continue;
        }
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if is_partial(name) {
                continue;
            }
        }
        if let Ok(modified) = meta.modified() {
            if SystemTime::now()
                .duration_since(modified)
                .unwrap_or(Duration::from_secs(0))
                < min_age
            {
                continue;
            }
        }
        let mut applied: Option<(&Rule, PathBuf)> = None;
        for rule in &cfg.rules {
            if matches_rule(&path, &meta, rule) {
                let target_dir = PathBuf::from(&rule.target_dir);
                ensure_dir(&target_dir)?;
                let target = target_dir.join(
                    path.file_name()
                        .map(|n| n.to_os_string())
                        .unwrap_or_default(),
                );
                let target = unique_target(&target);
                applied = Some((rule, target));
                break;
            }
        }
        if let Some((rule, target)) = applied {
            fs::rename(&path, &target)
                .with_context(|| format!("move {} -> {}", path.display(), target.display()))?;

            let mut symlink_info = None;
            if rule.create_symlink.unwrap_or(false) {
                #[cfg(windows)]
                let res = std::os::windows::fs::symlink_file(&target, &path);
                #[cfg(unix)]
                let res = std::os::unix::fs::symlink(&target, &path);
                
                match res {
                    Ok(_) => {
                        symlink_info = Some("Symlink created".to_string());
                        #[cfg(windows)]
                        {
                            let _ = std::process::Command::new("attrib")
                                .arg("+h")
                                .arg(&path)
                                .arg("/L")
                                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                                .status();
                        }
                    }
                    Err(e) => symlink_info = Some(format!("Symlink failed: {}", e)),
                }
            }

            actions.push((path, target.clone(), rule.name.clone(), symlink_info));
        }
    }
    Ok(actions)
}

pub fn watch_polling<F>(cfg: &DownloadsConfig, interval_secs: u64, callback: F) -> Result<()>
where
    F: Fn(&[(PathBuf, PathBuf, String, Option<String>)]),
{
    loop {
        match organize_once(cfg) {
            Ok(actions) => {
                if !actions.is_empty() {
                    callback(&actions);
                }
            }
            Err(e) => eprintln!("organize error: {}", e),
        }
        thread::sleep(Duration::from_secs(interval_secs));
    }
}

fn expand_env(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    let bytes = input.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if let Some(end) = input[i + 1..].find('%') {
                let var = &input[i + 1..i + 1 + end];
                let val = std::env::var(var).unwrap_or_else(|_| "".to_string());
                out.push_str(&val);
                i += end + 2;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

pub fn cleanup_old_symlinks(cfg: &DownloadsConfig) -> Result<usize> {
    let base = PathBuf::from(&cfg.download_dir);
    if !base.exists() {
        return Ok(0);
    }
    
    let mut count = 0;
    // Collect target dirs to check against
    let target_dirs: Vec<PathBuf> = cfg.rules.iter()
        .map(|r| PathBuf::from(&r.target_dir))
        .collect();

    for entry in fs::read_dir(&base).with_context(|| format!("list {}", base.display()))? {
        let entry = entry?;
        let path = entry.path();
        
        let meta = match fs::symlink_metadata(&path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        if meta.file_type().is_symlink() {
            // Check if it points to one of our folders
            if let Ok(target) = fs::read_link(&path) {
                // If relative symlink, resolve it relative to base
                let abs_target = if target.is_relative() {
                    base.join(&target)
                } else {
                    target
                };

                let points_to_our_dir = target_dirs.iter().any(|d| abs_target.starts_with(d));
                
                if points_to_our_dir {
                    // It's one of ours, delete it
                    if fs::remove_file(&path).is_ok() {
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}
