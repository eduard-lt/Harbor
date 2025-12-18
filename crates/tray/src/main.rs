use anyhow::Result;
use harbor_core::downloads::{
    load_downloads_config, organize_once, watch_polling, DownloadsConfig,
};
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;

extern crate native_windows_gui as nwg;

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
        let _ = watch_polling(&cfg, 5);
        w.store(false, Ordering::SeqCst);
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
        let _ = std::process::Command::new("explorer")
            .arg(path)
            .spawn();
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

fn append_recent(actions: &[(PathBuf, PathBuf, String)]) {
    if actions.is_empty() {
        return;
    }
    let dir = local_appdata_harbor();
    let _ = std::fs::create_dir_all(&dir);
    let log = recent_log_path();
    let mut buf = String::new();
    for (from, to, rule) in actions {
        buf.push_str(&format!(
            "{} -> {} ({})\n",
            from.display(),
            to.display(),
            rule
        ));
    }
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log)
        .and_then(|mut f| std::io::Write::write_all(&mut f, buf.as_bytes()));
}

fn main() -> Result<()> {
    nwg::init()?;

    let cfg_path = PathBuf::from("harbor.downloads.yaml");
    let cfg = if cfg_path.exists() {
        load_downloads_config(&cfg_path)?
    } else {
        load_downloads_config(PathBuf::from("examples/harbor.downloads.yaml"))?
    };
    let watching = Arc::new(AtomicBool::new(false));
    let handle: Arc<Mutex<Option<thread::JoinHandle<()>>>> = Arc::new(Mutex::new(None));

    let cfg_arc = Arc::new(cfg);

    let mut icon = nwg::Icon::default();
    nwg::Icon::builder()
        .source_system(Some(nwg::OemIcon::Information))
        .build(&mut icon)?;

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

    start_watching(&watching, &cfg_arc, &handle);

    nwg::dispatch_thread_events();
    Ok(())
}
