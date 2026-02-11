#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use harbor_core::downloads::{default_config, load_downloads_config, DownloadsConfig};
use harbor_core::types::Rule;

use state::AppState;
use std::path::PathBuf;
use tauri::{Emitter, Manager};

fn local_appdata_harbor() -> PathBuf {
    std::env::var("LOCALAPPDATA")
        .map(|p| PathBuf::from(p).join("Harbor"))
        .unwrap_or(PathBuf::from("C:\\Harbor"))
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

    // Start service by default
    let _ = commands::settings::internal_start_service(&app_state);

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
            commands::reset_to_defaults,
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .setup(|app| {
            use tauri::image::Image;
            use tauri::menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder};
            use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};

            // Load _h icon
            let icon_bytes = include_bytes!("../../../assets/icon_h.ico");
            let tray_icon = Image::from_bytes(icon_bytes).expect("Failed to load tray icon");

            // Build Tray Menu
            let status_on = CheckMenuItemBuilder::new("Service On")
                .id("service_on")
                .checked(true) // Default is on
                .build(app)?;

            let status_off = CheckMenuItemBuilder::new("Service Off")
                .id("service_off")
                .checked(false)
                .build(app)?;

            let organize_now = MenuItemBuilder::new("Organize Now")
                .id("organize")
                .build(app)?;

            let open_downloads = MenuItemBuilder::new("Open Downloads")
                .id("open_downloads")
                .build(app)?;
            let open_rules = MenuItemBuilder::new("Open Rules")
                .id("open_rules")
                .build(app)?; // Will open app at rules
            let open_activity = MenuItemBuilder::new("Open Recent Moves")
                .id("open_activity")
                .build(app)?;
            let open_settings = MenuItemBuilder::new("Settings")
                .id("open_settings")
                .build(app)?;

            let quit_i = MenuItemBuilder::new("Quit").id("quit").build(app)?;

            let menu = MenuBuilder::new(app)
                .items(&[
                    &status_on,
                    &status_off,
                    &tauri::menu::PredefinedMenuItem::separator(app)?,
                    &organize_now,
                    &tauri::menu::PredefinedMenuItem::separator(app)?,
                    &open_downloads,
                    &open_rules,
                    &open_activity,
                    &open_settings,
                    &tauri::menu::PredefinedMenuItem::separator(app)?,
                    &quit_i,
                ])
                .build()?;

            let _tray = TrayIconBuilder::with_id("tray")
                .icon(tray_icon)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "service_on" => {
                        let state: tauri::State<AppState> = app.state();
                        let _ = commands::settings::internal_start_service(&state);
                        let _ = status_on.set_checked(true);
                        let _ = status_off.set_checked(false);
                        // Force update UI if open? Not easy from here, UI polls status.
                    }
                    "service_off" => {
                        let state: tauri::State<AppState> = app.state();
                        let _ = commands::settings::internal_stop_service(&state);
                        let _ = status_on.set_checked(false);
                        let _ = status_off.set_checked(true);
                    }
                    "organize" => {
                        let app_handle = app.clone();
                        tauri::async_runtime::spawn(async move {
                            let state: tauri::State<AppState> = app_handle.state();
                            let _ = commands::trigger_organize_now(state).await;
                        });
                    }
                    "open_downloads" => {
                        let app_handle = app.clone();
                        tauri::async_runtime::spawn(async move {
                            let state: tauri::State<AppState> = app_handle.state();
                            let _ = commands::open_downloads_folder(state).await;
                        });
                    }
                    "open_rules" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = window.eval("window.location.href = '/rules'"); // Simple way to nav? Or use an event.
                                                                                    // Better: emit event to frontend router.
                            let _ = window.emit("navigate", "/rules");
                        }
                    }
                    "open_activity" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = window.emit("navigate", "/");
                        }
                    }
                    "open_settings" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = window.emit("navigate", "/settings");
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
