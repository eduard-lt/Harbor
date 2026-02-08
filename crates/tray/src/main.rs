#![windows_subsystem = "windows"]

use anyhow::Result;
use harbor_core::downloads::{
    cleanup_old_symlinks, load_downloads_config, organize_once, watch_polling, DownloadsConfig,
    Rule,
};
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use windows::{core::PCWSTR, Win32::Foundation::*, Win32::System::Threading::*};

extern crate native_windows_gui as nwg;

// Single instance checker using Windows mutex
struct SingleInstance {
    _mutex_handle: HANDLE,
}

impl SingleInstance {
    fn new(name: &str) -> Result<Self> {
        unsafe {
            let mutex_name = format!("Global\\{}", name);
            let wide_name: Vec<u16> = mutex_name
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();

            let mutex_handle = CreateMutexW(
                None,
                true, // Initially owned
                PCWSTR(wide_name.as_ptr()),
            )?;

            // Check if mutex already existed
            let last_error = GetLastError();
            if last_error.0 == ERROR_ALREADY_EXISTS.0 {
                // Another instance is running
                let _ = CloseHandle(mutex_handle);
                anyhow::bail!("Another instance of Harbor is already running");
            }

            Ok(Self {
                _mutex_handle: mutex_handle,
            })
        }
    }
}

impl Drop for SingleInstance {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self._mutex_handle);
        }
    }
}

struct TrayState {
    window: nwg::MessageWindow,
    tray: nwg::TrayNotification,
    tray_menu: nwg::Menu,
    item_start: nwg::MenuItem,
    item_stop: nwg::MenuItem,
    item_organize: nwg::MenuItem,
    item_open_downloads: nwg::MenuItem,
    item_open_cfg: nwg::MenuItem,
    item_open_recent: nwg::MenuItem,
    item_exit: nwg::MenuItem,
}

fn show_menu(ui: &TrayState) {
    let (x, y) = nwg::GlobalCursor::position();
    nwg::Menu::popup(&ui.tray_menu, x, y);
}

fn start_watching(
    watching: &Arc<AtomicBool>,
    cfg: &DownloadsConfig,
    handle: &Arc<Mutex<Option<thread::JoinHandle<()>>>>,
) {
    if watching.swap(true, Ordering::SeqCst) {
        return;
    }
    let cfg = cfg.clone();
    let w = watching.clone();
    let h = thread::spawn(move || {
        let _ = watch_polling(&cfg, 5, &w, |actions| {
            append_recent(actions);
        });
    });
    let mut guard = handle.lock().unwrap();
    *guard = Some(h);
}

fn stop_watching(watching: &Arc<AtomicBool>, handle: &Arc<Mutex<Option<thread::JoinHandle<()>>>>) {
    watching.store(false, Ordering::SeqCst);
    let mut guard = handle.lock().unwrap();
    if let Some(h) = guard.take() {
        #[allow(clippy::disallowed_methods)]
        let _ = h.thread().id();
        // The watch loop checks the flag each cycle; to stop promptly, we rely on short intervals.
    }
}

fn open_config(path: &PathBuf) {
    if cfg!(windows) {
        let _ = std::process::Command::new("cmd")
            .arg("/C")
            .arg("start")
            .arg(path)
            .spawn();
    }
}

fn open_folder(path: &PathBuf) {
    if cfg!(windows) {
        let _ = std::process::Command::new("explorer").arg(path).spawn();
    }
}

fn local_appdata_harbor() -> PathBuf {
    std::env::var("LOCALAPPDATA")
        .map(|p| PathBuf::from(p).join("Harbor"))
        .unwrap_or(PathBuf::from("C:\\Harbor"))
}

fn recent_log_path() -> PathBuf {
    local_appdata_harbor().join("recent_moves.log")
}

fn append_recent(actions: &[(PathBuf, PathBuf, String, Option<String>)]) {
    if actions.is_empty() {
        return;
    }
    let dir = local_appdata_harbor();
    let _ = std::fs::create_dir_all(&dir);
    let log = recent_log_path();
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
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log)
        .and_then(|mut f| std::io::Write::write_all(&mut f, buf.as_bytes()));
}

fn main() -> Result<()> {
    // Ensure only one instance of Harbor is running
    let _instance = SingleInstance::new("Harbor-Tray-Instance")?;

    nwg::init()?;

    let cfg_path = local_appdata_harbor().join("harbor.downloads.yaml");

    // If config doesn't exist, try to copy from default template
    if !cfg_path.exists() {
        let default_config = local_appdata_harbor().join("harbor.downloads.yaml.default");
        if default_config.exists() {
            // Copy the default config to the active config
            let _ = std::fs::copy(&default_config, &cfg_path);
        }
    }

    let cfg = if cfg_path.exists() {
        load_downloads_config(&cfg_path)?
    } else {
        let user = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Public".to_string());
        let dl = format!("{}\\Downloads", user);
        let pictures = format!("{}\\Downloads\\Images", user);
        let videos = format!("{}\\Downloads\\Videos", user);
        let music = format!("{}\\Downloads\\Music", user);
        let docs = format!("{}\\Downloads\\Documents", user);
        let archives = format!("{}\\Downloads\\Archives", user);
        let installers = format!("{}\\Downloads\\Installers", user);
        let torrents = format!("{}\\Downloads\\Torrents", user);
        let webpages = format!("{}\\Downloads\\Webpages", user);
        let isos = format!("{}\\Downloads\\ISOs", user);
        let dev = format!("{}\\Downloads\\Dev", user);
        let subtitles = format!("{}\\Downloads\\Subtitles", user);
        DownloadsConfig {
            download_dir: dl,
            min_age_secs: Some(5),
            rules: vec![
                Rule {
                    name: "Images".to_string(),
                    extensions: Some(
                        [
                            "jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "heic", "svg",
                            "avif",
                        ]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                    ),
                    pattern: None,
                    min_size_bytes: None,
                    max_size_bytes: None,
                    target_dir: pictures,
                    create_symlink: None,
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
                },
                Rule {
                    name: "Documents".to_string(),
                    extensions: Some(
                        [
                            "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "rtf",
                        ]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                    ),
                    pattern: None,
                    min_size_bytes: None,
                    max_size_bytes: None,
                    target_dir: docs.clone(),
                    create_symlink: None,
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
                },
                Rule {
                    name: "ISOs".to_string(),
                    extensions: Some(["iso"].iter().map(|s| s.to_string()).collect()),
                    pattern: None,
                    min_size_bytes: None,
                    max_size_bytes: None,
                    target_dir: isos,
                    create_symlink: None,
                },
                Rule {
                    name: "Torrents".to_string(),
                    extensions: Some(["torrent"].iter().map(|s| s.to_string()).collect()),
                    pattern: None,
                    min_size_bytes: None,
                    max_size_bytes: None,
                    target_dir: torrents,
                    create_symlink: None,
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
                },
                Rule {
                    name: "Data".to_string(),
                    extensions: Some(["csv"].iter().map(|s| s.to_string()).collect()),
                    pattern: None,
                    min_size_bytes: None,
                    max_size_bytes: None,
                    target_dir: docs,
                    create_symlink: None,
                },
                Rule {
                    name: "Web Pages".to_string(),
                    extensions: Some(["html", "htm"].iter().map(|s| s.to_string()).collect()),
                    pattern: None,
                    min_size_bytes: None,
                    max_size_bytes: None,
                    target_dir: webpages,
                    create_symlink: None,
                },
                Rule {
                    name: "Subtitles".to_string(),
                    extensions: Some(["srt"].iter().map(|s| s.to_string()).collect()),
                    pattern: None,
                    min_size_bytes: None,
                    max_size_bytes: None,
                    target_dir: subtitles,
                    create_symlink: None,
                },
            ],
        }
    };
    let watching = Arc::new(AtomicBool::new(false));
    let handle: Arc<Mutex<Option<thread::JoinHandle<()>>>> = Arc::new(Mutex::new(None));

    let cfg_arc = Arc::new(cfg);

    let mut icon = nwg::Icon::default();
    let icon_h = local_appdata_harbor().join("icon_h.ico");
    let tray_ico = local_appdata_harbor().join("harbor-tray.ico");
    let app_ico = local_appdata_harbor().join("harbor.ico");
    if icon_h.exists() {
        nwg::Icon::builder()
            .source_file(Some(icon_h.to_string_lossy().as_ref()))
            .build(&mut icon)?;
    } else if tray_ico.exists() {
        nwg::Icon::builder()
            .source_file(Some(tray_ico.to_string_lossy().as_ref()))
            .build(&mut icon)?;
    } else if app_ico.exists() {
        nwg::Icon::builder()
            .source_file(Some(app_ico.to_string_lossy().as_ref()))
            .build(&mut icon)?;
    } else {
        nwg::Icon::builder()
            .source_system(Some(nwg::OemIcon::Information))
            .build(&mut icon)?;
    }

    let mut window = nwg::MessageWindow::default();
    nwg::MessageWindow::builder().build(&mut window)?;

    let mut tray = nwg::TrayNotification::default();
    nwg::TrayNotification::builder()
        .parent(&window)
        .icon(Some(&icon))
        .tip(Some("Harbor Downloads Organizer"))
        .build(&mut tray)?;

    let mut tray_menu = nwg::Menu::default();
    nwg::Menu::builder()
        .popup(true)
        .parent(&window)
        .build(&mut tray_menu)?;

    let mut item_start = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .text("Start Watching")
        .check(true)
        .parent(&tray_menu)
        .build(&mut item_start)?;

    let mut item_stop = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .text("Stop Watching")
        .check(false)
        .parent(&tray_menu)
        .build(&mut item_stop)?;

    let mut item_organize = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .text("Organize Now")
        .parent(&tray_menu)
        .build(&mut item_organize)?;

    let mut item_open_downloads = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .text("Open Downloads")
        .parent(&tray_menu)
        .build(&mut item_open_downloads)?;

    let mut item_open_cfg = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .text("Open Config")
        .parent(&tray_menu)
        .build(&mut item_open_cfg)?;

    let mut item_open_recent = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .text("Open Recent Moves")
        .parent(&tray_menu)
        .build(&mut item_open_recent)?;

    let mut item_exit = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .text("Exit")
        .parent(&tray_menu)
        .build(&mut item_exit)?;

    let ui = TrayState {
        window,
        tray,
        tray_menu,
        item_start,
        item_stop,
        item_organize,
        item_open_downloads,
        item_open_cfg,
        item_open_recent,
        item_exit,
    };

    let ui_ref = std::rc::Rc::new(ui);
    let ui_weak = std::rc::Rc::downgrade(&ui_ref);
    let cfg_open_path = cfg_path.clone();
    let downloads_dir = PathBuf::from(&cfg_open_path)
        .parent()
        .map(|_| PathBuf::from(&cfg_arc.download_dir))
        .unwrap_or(PathBuf::from(&cfg_arc.download_dir));
    let watching_c = watching.clone();
    let handle_c = handle.clone();
    let cfg_arc_c = cfg_arc.clone();

    let handler = move |evt, _evt_data, handle| {
        if let Some(ui) = ui_weak.upgrade() {
            match evt {
                nwg::Event::OnContextMenu => {
                    if handle == ui.tray {
                        show_menu(&ui);
                    }
                }
                nwg::Event::OnMenuItemSelected => {
                    if handle == ui.item_start {
                        start_watching(&watching_c, &cfg_arc_c, &handle_c);
                    } else if handle == ui.item_stop {
                        stop_watching(&watching_c, &handle_c);
                    } else if handle == ui.item_organize {
                        if let Ok(actions) = organize_once(&cfg_arc_c) {
                            append_recent(&actions);
                            if !actions.is_empty() {
                                ui.tray.show(
                                    &format!("Moved {} file(s)", actions.len()),
                                    Some("Harbor"),
                                    Some(nwg::TrayNotificationFlags::INFO_ICON),
                                    None,
                                );
                            }
                        }
                    } else if handle == ui.item_open_downloads {
                        open_folder(&downloads_dir);
                    } else if handle == ui.item_open_cfg {
                        open_config(&cfg_open_path);
                    } else if handle == ui.item_open_recent {
                        let p = recent_log_path();
                        if !p.exists() {
                            if let Some(parent) = p.parent() {
                                let _ = std::fs::create_dir_all(parent);
                            }
                            let _ = std::fs::write(&p, "Recent Moves Log\n----------------\n");
                        }
                        open_config(&p);
                    } else if handle == ui.item_exit {
                        nwg::stop_thread_dispatch();
                    }
                }
                _ => {}
            }
        }
    };
    let _eh = nwg::full_bind_event_handler(&ui_ref.window.handle, handler);

    // Cleanup old symlinks on startup
    if let Ok(count) = cleanup_old_symlinks(&cfg_arc) {
        if count > 0 {
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(recent_log_path())
                .and_then(|mut f| {
                    use std::io::Write;
                    writeln!(f, "Startup: Cleaned up {} old symlink(s)", count)
                });
        }
    }

    start_watching(&watching, &cfg_arc, &handle);

    nwg::dispatch_thread_events();
    Ok(())
}
