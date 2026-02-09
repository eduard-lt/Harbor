use crate::state::AppState;
use harbor_core::downloads::{load_downloads_config, organize_once, watch_polling};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::thread;
use tauri::State;

#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;

/// Service status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub running: bool,
    pub uptime_seconds: Option<u64>,
}

fn append_to_log(log_path: &PathBuf, actions: &[(PathBuf, PathBuf, String, Option<String>)]) {
    if actions.is_empty() {
        return;
    }

    if let Some(parent) = log_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut buf = String::new();
    for (from, to, rule, symlink_info) in actions {
        let symlink_msg = symlink_info.as_deref().unwrap_or("");
        buf.push_str(&format!(
            "{} -> {} ({}) {}\n",
            from.display(),
            to.display(),
            rule,
            symlink_msg
        ));
    }

    if let Ok(mut file) = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
    {
        let _ = file.write_all(buf.as_bytes());
    }
}

#[tauri::command]
pub async fn get_service_status(state: State<'_, AppState>) -> Result<ServiceStatus, String> {
    let running = state.watching.load(Ordering::SeqCst);

    Ok(ServiceStatus {
        running,
        uptime_seconds: None, // Could track this if needed
    })
}

#[tauri::command]
pub async fn start_service(state: State<'_, AppState>) -> Result<(), String> {
    if state.watching.swap(true, Ordering::SeqCst) {
        // Already running
        return Ok(());
    }

    let config = state.config.read().map_err(|e| e.to_string())?.clone();
    let watching = state.watching.clone();
    let log_path = state.recent_log_path();

    let handle = thread::spawn(move || {
        let _ = watch_polling(&config, 5, &watching, |actions| {
            append_to_log(&log_path, actions);
        });
    });

    let mut guard = state.watcher_handle.lock().map_err(|e| e.to_string())?;
    *guard = Some(handle);

    Ok(())
}

#[tauri::command]
pub async fn stop_service(state: State<'_, AppState>) -> Result<(), String> {
    state.watching.store(false, Ordering::SeqCst);

    // The watcher thread will exit on next iteration
    let mut guard = state.watcher_handle.lock().map_err(|e| e.to_string())?;
    *guard = None;

    Ok(())
}

#[tauri::command]
pub async fn trigger_organize_now(state: State<'_, AppState>) -> Result<usize, String> {
    let config = state.config.read().map_err(|e| e.to_string())?.clone();
    let log_path = state.recent_log_path();

    let actions = organize_once(&config).map_err(|e| format!("Organize failed: {}", e))?;

    append_to_log(&log_path, &actions);

    Ok(actions.len())
}

#[tauri::command]
pub async fn get_startup_enabled() -> Result<bool, String> {
    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let run_key = hkcu
            .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run")
            .map_err(|e| format!("Failed to open registry key: {}", e))?;

        match run_key.get_value::<String, _>("Harbor") {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    #[cfg(not(windows))]
    {
        Ok(false)
    }
}

#[tauri::command]
pub async fn set_startup_enabled(enabled: bool) -> Result<(), String> {
    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (run_key, _) = hkcu
            .create_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run")
            .map_err(|e| format!("Failed to open registry key: {}", e))?;

        if enabled {
            // Get the path to the tray executable
            let exe_path = std::env::current_exe()
                .map_err(|e| format!("Failed to get executable path: {}", e))?;

            run_key
                .set_value("Harbor", &exe_path.to_string_lossy().as_ref())
                .map_err(|e| format!("Failed to set registry value: {}", e))?;
        } else {
            // Remove the startup entry
            let _ = run_key.delete_value("Harbor");
        }

        Ok(())
    }

    #[cfg(not(windows))]
    {
        Err("Startup configuration not supported on this platform".to_string())
    }
}

#[tauri::command]
pub async fn reload_config(state: State<'_, AppState>) -> Result<(), String> {
    let new_config =
        load_downloads_config(&state.config_path).map_err(|e| format!("Failed to reload config: {}", e))?;

    let mut config = state.config.write().map_err(|e| e.to_string())?;
    *config = new_config;

    Ok(())
}

#[tauri::command]
pub async fn open_config_file(state: State<'_, AppState>) -> Result<(), String> {
    let path = &state.config_path;

    #[cfg(windows)]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", path.to_string_lossy().as_ref()])
            .spawn()
            .map_err(|e| format!("Failed to open config file: {}", e))?;
    }

    #[cfg(not(windows))]
    {
        std::process::Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map_err(|e| format!("Failed to open config file: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn open_downloads_folder(state: State<'_, AppState>) -> Result<(), String> {
    let config = state.config.read().map_err(|e| e.to_string())?;
    let path = &config.download_dir;

    #[cfg(windows)]
    {
        std::process::Command::new("explorer")
            .arg(path)
            .spawn()
            .map_err(|e| format!("Failed to open downloads folder: {}", e))?;
    }

    #[cfg(not(windows))]
    {
        std::process::Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map_err(|e| format!("Failed to open downloads folder: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_config_path(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.config_path.to_string_lossy().to_string())
}
