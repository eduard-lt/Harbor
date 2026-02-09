#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use harbor_core::downloads::{load_downloads_config, DownloadsConfig, Rule};
use state::AppState;
use std::path::PathBuf;

fn local_appdata_harbor() -> PathBuf {
    std::env::var("LOCALAPPDATA")
        .map(|p| PathBuf::from(p).join("Harbor"))
        .unwrap_or(PathBuf::from("C:\\Harbor"))
}

fn default_config() -> DownloadsConfig {
    let user = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Public".to_string());
    let dl = format!("{}\\Downloads", user);
    let pictures = format!("{}\\Downloads\\Images", user);
    let videos = format!("{}\\Downloads\\Videos", user);
    let music = format!("{}\\Downloads\\Music", user);
    let docs = format!("{}\\Downloads\\Documents", user);
    let archives = format!("{}\\Downloads\\Archives", user);
    let installers = format!("{}\\Downloads\\Installers", user);
    let torrents = format!("{}\\Downloads\\Torrents", user);
    let isos = format!("{}\\Downloads\\ISOs", user);
    let dev = format!("{}\\Downloads\\Dev", user);
    let subtitles = format!("{}\\Downloads\\Subtitles", user);
    let webpages = format!("{}\\Downloads\\Webpages", user);

    DownloadsConfig {
        download_dir: dl,
        min_age_secs: Some(5),
        rules: vec![
            Rule {
                name: "Images".to_string(),
                extensions: Some(
                    ["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "heic", "svg", "avif"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                ),
                pattern: None,
                min_size_bytes: None,
                max_size_bytes: None,
                target_dir: pictures,
                create_symlink: None,
                enabled: Some(true),
            },
            Rule {
                name: "Videos".to_string(),
                extensions: Some(
                    ["mp4", "mkv", "avi", "mov", "wmv", "webm"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                ),
                pattern: None,
                min_size_bytes: None,
                max_size_bytes: None,
                target_dir: videos,
                create_symlink: None,
                enabled: Some(true),
            },
            Rule {
                name: "Music".to_string(),
                extensions: Some(
                    ["mp3", "flac", "wav", "aac", "ogg"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                ),
                pattern: None,
                min_size_bytes: None,
                max_size_bytes: None,
                target_dir: music,
                create_symlink: None,
                enabled: Some(true),
            },
            Rule {
                name: "Archives".to_string(),
                extensions: Some(
                    ["zip", "rar", "7z", "tar", "gz", "xz"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                ),
                pattern: None,
                min_size_bytes: None,
                max_size_bytes: None,
                target_dir: archives,
                create_symlink: None,
                enabled: Some(true),
            },
            Rule {
                name: "Documents".to_string(),
                extensions: Some(
                    ["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "rtf"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                ),
                pattern: None,
                min_size_bytes: None,
                max_size_bytes: None,
                target_dir: docs.clone(),
                create_symlink: None,
                enabled: Some(true),
            },
            Rule {
                name: "Installers".to_string(),
                extensions: Some(
                    ["exe", "msi", "msix", "dmg", "pkg", "apk"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                ),
                pattern: None,
                min_size_bytes: None,
                max_size_bytes: None,
                target_dir: installers,
                create_symlink: None,
                enabled: Some(true),
            },
            Rule {
                name: "ISOs".to_string(),
                extensions: Some(["iso"].iter().map(|s| s.to_string()).collect()),
                pattern: None,
                min_size_bytes: None,
                max_size_bytes: None,
                target_dir: isos,
                create_symlink: None,
                enabled: Some(true),
            },
            Rule {
                name: "Torrents".to_string(),
                extensions: Some(["torrent"].iter().map(|s| s.to_string()).collect()),
                pattern: None,
                min_size_bytes: None,
                max_size_bytes: None,
                target_dir: torrents,
                create_symlink: None,
                enabled: Some(true),
            },
            Rule {
                name: "Dev".to_string(),
                extensions: Some(
                    ["json", "env", "xml", "plist"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                ),
                pattern: None,
                min_size_bytes: None,
                max_size_bytes: None,
                target_dir: dev,
                create_symlink: None,
                enabled: Some(true),
            },
            Rule {
                name: "Web Pages".to_string(),
                extensions: Some(["html", "htm"].iter().map(|s| s.to_string()).collect()),
                pattern: None,
                min_size_bytes: None,
                max_size_bytes: None,
                target_dir: webpages,
                create_symlink: None,
                enabled: Some(true),
            },
            Rule {
                name: "Subtitles".to_string(),
                extensions: Some(["srt", "vtt"].iter().map(|s| s.to_string()).collect()),
                pattern: None,
                min_size_bytes: None,
                max_size_bytes: None,
                target_dir: subtitles,
                create_symlink: None,
                enabled: Some(true),
            },
        ],
    }
}

fn main() {
    let harbor_dir = local_appdata_harbor();
    let _ = std::fs::create_dir_all(&harbor_dir);

    let cfg_path = harbor_dir.join("harbor.downloads.yaml");

    // If config doesn't exist, try to copy from default template or create default
    if !cfg_path.exists() {
        let default_config_path = harbor_dir.join("harbor.downloads.yaml.default");
        if default_config_path.exists() {
            let _ = std::fs::copy(&default_config_path, &cfg_path);
        } else {
            // Create default config
            let config = default_config();
            if let Ok(yaml) = serde_yaml::to_string(&config) {
                let _ = std::fs::write(&cfg_path, yaml);
            }
        }
    }

    let config = if cfg_path.exists() {
        load_downloads_config(&cfg_path).unwrap_or_else(|_| default_config())
    } else {
        default_config()
    };

    let app_state = AppState::new(cfg_path, config);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Rules commands
            commands::get_rules,
            commands::create_rule,
            commands::update_rule,
            commands::delete_rule,
            commands::toggle_rule,
            commands::reorder_rules,
            commands::get_download_dir,
            // Activity commands
            commands::get_activity_logs,
            commands::get_activity_stats,
            commands::clear_activity_logs,
            // Settings commands
            commands::get_service_status,
            commands::start_service,
            commands::stop_service,
            commands::trigger_organize_now,
            commands::get_startup_enabled,
            commands::set_startup_enabled,
            commands::reload_config,
            commands::open_config_file,
            commands::open_downloads_folder,
            commands::get_config_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
