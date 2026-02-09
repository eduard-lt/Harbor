use harbor_core::downloads::DownloadsConfig;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::JoinHandle;

/// Application state managed by Tauri
pub struct AppState {
    /// Path to the configuration file
    pub config_path: PathBuf,
    /// Whether the file watcher is currently active
    pub watching: Arc<AtomicBool>,
    /// Current configuration (cached)
    pub config: Arc<RwLock<DownloadsConfig>>,
    /// Handle to the watcher thread
    pub watcher_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl AppState {
    pub fn new(config_path: PathBuf, config: DownloadsConfig) -> Self {
        Self {
            config_path,
            watching: Arc::new(AtomicBool::new(false)),
            config: Arc::new(RwLock::new(config)),
            watcher_handle: Arc::new(Mutex::new(None)),
        }
    }

    /// Get the path to the recent moves log
    pub fn recent_log_path(&self) -> PathBuf {
        self.config_path
            .parent()
            .unwrap_or(&self.config_path)
            .join("recent_moves.log")
    }

    /// Get the Harbor data directory
    pub fn data_dir(&self) -> PathBuf {
        self.config_path
            .parent()
            .unwrap_or(&self.config_path)
            .to_path_buf()
    }
}
