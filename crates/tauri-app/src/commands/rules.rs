use crate::commands::settings::{internal_start_service, internal_stop_service};
use crate::state::AppState;
use harbor_core::downloads::DownloadsConfig;
use harbor_core::types::Rule;

use serde::{Deserialize, Serialize};
use std::fs;
use tauri::State;

/// Frontend-facing rule representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDto {
    /// Rule name (used as ID)
    pub id: String,
    /// Display name
    pub name: String,
    /// File extensions this rule applies to
    pub extensions: Vec<String>,
    /// Optional regex pattern for filename matching
    pub pattern: Option<String>,
    /// Minimum file size in bytes
    pub min_size_bytes: Option<u64>,
    /// Maximum file size in bytes
    pub max_size_bytes: Option<u64>,
    /// Target directory for matched files
    pub destination: String,
    /// Whether to create a symlink in the original location
    pub create_symlink: bool,
    /// Whether the rule is enabled
    pub enabled: bool,
    /// Icon name (derived from first extension)
    pub icon: String,
    /// Icon color
    pub icon_color: String,
}

impl From<&Rule> for RuleDto {
    fn from(rule: &Rule) -> Self {
        let icon = derive_icon(rule.extensions.as_ref());
        let icon_color = derive_icon_color(rule.extensions.as_ref());

        RuleDto {
            id: rule.name.clone(),
            name: rule.name.clone(),
            extensions: rule
                .extensions
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|e| format!(".{}", e))
                .collect(),
            pattern: rule.pattern.clone(),
            min_size_bytes: rule.min_size_bytes,
            max_size_bytes: rule.max_size_bytes,
            destination: rule.target_dir.clone(),
            create_symlink: rule.create_symlink.unwrap_or(false),
            enabled: rule.enabled.unwrap_or(true),
            icon,
            icon_color,
        }
    }
}

fn derive_icon(extensions: Option<&Vec<String>>) -> String {
    let ext = extensions
        .and_then(|e| e.first())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "bmp" | "tiff" | "heic" | "avif" => {
            "image".to_string()
        }
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "webm" => "movie".to_string(),
        "mp3" | "flac" | "wav" | "aac" | "ogg" => "music_note".to_string(),
        "pdf" | "doc" | "docx" | "txt" | "rtf" => "description".to_string(),
        "xls" | "xlsx" | "csv" => "table_chart".to_string(),
        "ppt" | "pptx" => "slideshow".to_string(),
        "zip" | "rar" | "7z" | "tar" | "gz" | "xz" => "folder_zip".to_string(),
        "exe" | "msi" | "msix" | "dmg" | "pkg" | "apk" => "install_desktop".to_string(),
        "iso" => "album".to_string(),
        "torrent" => "download".to_string(),
        "html" | "htm" => "web".to_string(),
        "json" | "xml" | "yaml" | "yml" => "code".to_string(),
        "srt" | "vtt" => "subtitles".to_string(),
        _ => "insert_drive_file".to_string(),
    }
}

fn derive_icon_color(extensions: Option<&Vec<String>>) -> String {
    let ext = extensions
        .and_then(|e| e.first())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "bmp" | "tiff" | "heic" | "avif" => {
            "indigo".to_string()
        }
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "webm" => "purple".to_string(),
        "mp3" | "flac" | "wav" | "aac" | "ogg" => "pink".to_string(),
        "pdf" | "doc" | "docx" | "txt" | "rtf" | "xls" | "xlsx" | "csv" | "ppt" | "pptx" => {
            "amber".to_string()
        }
        "zip" | "rar" | "7z" | "tar" | "gz" | "xz" => "slate".to_string(),
        "exe" | "msi" | "msix" | "dmg" | "pkg" | "apk" => "red".to_string(),
        _ => "slate".to_string(),
    }
}

fn save_config(state: &AppState, config: &DownloadsConfig) -> Result<(), String> {
    let yaml =
        serde_yaml::to_string(config).map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(&state.config_path, yaml).map_err(|e| format!("Failed to write config: {}", e))?;
    Ok(())
}

fn restart_service_if_running(state: &AppState) -> Result<(), String> {
    let flag_guard = state.watcher_flag.lock().map_err(|e| e.to_string())?;
    let is_running = flag_guard.is_some();
    drop(flag_guard);

    if is_running {
        internal_stop_service(state)?;
        internal_start_service(state)?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_rules(state: State<'_, AppState>) -> Result<Vec<RuleDto>, String> {
    let config = state.config.read().map_err(|e| e.to_string())?;
    Ok(config.rules.iter().map(RuleDto::from).collect())
}

#[tauri::command]
pub async fn create_rule(
    state: State<'_, AppState>,
    name: String,
    extensions: Vec<String>,
    destination: String,
    pattern: Option<String>,
    min_size_bytes: Option<u64>,
    max_size_bytes: Option<u64>,
    create_symlink: Option<bool>,
    enabled: Option<bool>,
) -> Result<RuleDto, String> {
    let new_rule = {
        let mut config = state.config.write().map_err(|e| e.to_string())?;

        // Check if rule with this name already exists
        if config.rules.iter().any(|r| r.name == name) {
            return Err(format!("Rule with name '{}' already exists", name));
        }

        // Convert extensions: remove leading dots if present
        let extensions: Vec<String> = extensions
            .into_iter()
            .map(|e| e.trim_start_matches('.').to_string())
            .filter(|e| !e.is_empty())
            .collect();

        let rule = Rule {
            name: name.clone(),
            extensions: if extensions.is_empty() {
                None
            } else {
                Some(extensions)
            },
            pattern,
            min_size_bytes,
            max_size_bytes,
            target_dir: destination,
            create_symlink,
            enabled,
        };

        config.rules.push(rule.clone());
        save_config(&state, &config)?;
        rule
    };

    restart_service_if_running(&state)?;

    Ok(RuleDto::from(&new_rule))
}

#[tauri::command]
pub async fn update_rule(
    state: State<'_, AppState>,
    id: String,
    name: Option<String>,
    extensions: Option<Vec<String>>,
    destination: Option<String>,
    pattern: Option<String>,
    min_size_bytes: Option<u64>,
    max_size_bytes: Option<u64>,
    create_symlink: Option<bool>,
    enabled: Option<bool>,
) -> Result<RuleDto, String> {
    let updated = {
        let mut config = state.config.write().map_err(|e| e.to_string())?;

        let rule = config
            .rules
            .iter_mut()
            .find(|r| r.name == id)
            .ok_or_else(|| format!("Rule '{}' not found", id))?;

        if let Some(new_name) = name {
            rule.name = new_name;
        }
        if let Some(exts) = extensions {
            let exts: Vec<String> = exts
                .into_iter()
                .map(|e| e.trim_start_matches('.').to_string())
                .filter(|e| !e.is_empty())
                .collect();
            rule.extensions = if exts.is_empty() { None } else { Some(exts) };
        }
        if let Some(dest) = destination {
            rule.target_dir = dest;
        }
        if pattern.is_some() {
            rule.pattern = pattern;
        }
        if min_size_bytes.is_some() {
            rule.min_size_bytes = min_size_bytes;
        }
        if max_size_bytes.is_some() {
            rule.max_size_bytes = max_size_bytes;
        }
        if let Some(symlink) = create_symlink {
            rule.create_symlink = Some(symlink);
        }
        if let Some(en) = enabled {
            rule.enabled = Some(en);
        }

        let updated = RuleDto::from(&*rule);
        save_config(&state, &config)?;
        updated
    };

    restart_service_if_running(&state)?;

    Ok(updated)
}

#[tauri::command]
pub async fn delete_rule(state: State<'_, AppState>, rule_name: String) -> Result<(), String> {
    {
        let mut config = state.config.write().map_err(|e| e.to_string())?;

        let original_len = config.rules.len();
        config.rules.retain(|r| r.name != rule_name);

        if config.rules.len() == original_len {
            return Err(format!("Rule '{}' not found", rule_name));
        }

        save_config(&state, &config)?;
    }
    restart_service_if_running(&state)?;
    Ok(())
}

#[tauri::command]
pub async fn toggle_rule(
    state: State<'_, AppState>,
    rule_name: String,
    enabled: bool,
) -> Result<(), String> {
    {
        let mut config = state.config.write().map_err(|e| e.to_string())?;

        let rule = config
            .rules
            .iter_mut()
            .find(|r| r.name == rule_name)
            .ok_or_else(|| format!("Rule '{}' not found", rule_name))?;

        rule.enabled = Some(enabled);
        save_config(&state, &config)?;
    }
    restart_service_if_running(&state)?;

    Ok(())
}

#[tauri::command]
pub async fn reorder_rules(
    state: State<'_, AppState>,
    rule_names: Vec<String>,
) -> Result<(), String> {
    {
        let mut config = state.config.write().map_err(|e| e.to_string())?;

        // Reorder rules based on the provided order
        let mut new_rules: Vec<Rule> = Vec::with_capacity(rule_names.len());

        for name in &rule_names {
            if let Some(rule) = config.rules.iter().find(|r| &r.name == name).cloned() {
                new_rules.push(rule);
            }
        }

        // Add any rules that weren't in the provided list (shouldn't happen, but safety first)
        for rule in &config.rules {
            if !rule_names.contains(&rule.name) {
                new_rules.push(rule.clone());
            }
        }

        config.rules = new_rules;
        save_config(&state, &config)?;
    }
    restart_service_if_running(&state)?;

    Ok(())
}

#[tauri::command]
pub async fn get_download_dir(state: State<'_, AppState>) -> Result<String, String> {
    let config = state.config.read().map_err(|e| e.to_string())?;
    Ok(config.download_dir.clone())
}
