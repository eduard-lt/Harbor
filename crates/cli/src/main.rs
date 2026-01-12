use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
#[cfg(windows)]
use winreg::enums::HKEY_CURRENT_USER;
#[cfg(windows)]
use winreg::RegKey;

#[derive(Parser)]
#[command(name = "harbor")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    DownloadsInit {
        #[arg(default_value = "harbor.downloads.yaml")]
        path: String,
    },
    DownloadsOrganize {
        #[arg(default_value = "harbor.downloads.yaml")]
        path: String,
    },
    DownloadsWatch {
        #[arg(default_value = "harbor.downloads.yaml")]
        path: String,
        #[arg(default_value_t = 5)]
        interval_secs: u64,
    },
    Validate {
        #[arg(default_value = "harbor.config.yaml")]
        path: String,
    },
    Init {
        #[arg(default_value = "harbor.config.yaml")]
        path: String,
    },
    Up {
        #[arg(default_value = "harbor.config.yaml")]
        path: String,
        #[arg(default_value = ".")]
        base_dir: String,
        #[arg(default_value = "harbor_state.json")]
        state_path: String,
    },
    Down {
        #[arg(default_value = "harbor_state.json")]
        state_path: String,
    },
    Status {
        #[arg(default_value = "harbor_state.json")]
        state_path: String,
    },
    Logs {
        service: String,
        #[arg(default_value = "logs")]
        logs_dir: String,
        #[arg(default_value = "stdout")]
        stream: String,
    },
    TrayInstall {
        #[arg(long)]
        source: Option<String>,
    },
    TrayUninstall,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::DownloadsInit { path } => {
            init_downloads_config(&path)?;
            Ok(())
        }
        Commands::DownloadsOrganize { path } => {
            let cfg = harbor_core::downloads::load_downloads_config(&path)?;
            let actions = harbor_core::downloads::organize_once(&cfg)?;
            for (from, to, rule, symlink_info) in actions {
                let sym = symlink_info.unwrap_or_default();
                println!("{} -> {} ({}) {}", from.display(), to.display(), rule, sym);
            }
            Ok(())
        }
        Commands::DownloadsWatch {
            path,
            interval_secs,
        } => {
            let cfg = harbor_core::downloads::load_downloads_config(&path)?;
            harbor_core::downloads::watch_polling(&cfg, interval_secs, |actions| {
                for (from, to, rule, symlink_info) in actions {
                    let sym = symlink_info.as_deref().unwrap_or_default();
                    println!("{} -> {} ({}) {}", from.display(), to.display(), rule, sym);
                }
            })?;
            Ok(())
        }
        Commands::Validate { path } => {
            let cfg = harbor_core::config::load_config(&path)?;
            harbor_core::config::validate_config(&cfg)?;
            println!("valid");
            Ok(())
        }
        Commands::Init { path } => init_config(&path),
        Commands::Up {
            path,
            base_dir,
            state_path,
        } => {
            let cfg = harbor_core::config::load_config(&path)?;
            harbor_core::config::validate_config(&cfg)?;
            let st = harbor_core::orchestrator::up(
                &cfg,
                PathBuf::from(base_dir),
                PathBuf::from(state_path),
            )?;
            println!("{}", serde_json::to_string_pretty(&st)?);
            Ok(())
        }
        Commands::Down { state_path } => {
            harbor_core::orchestrator::down(PathBuf::from(state_path))?;
            println!("down");
            Ok(())
        }
        Commands::Status { state_path } => {
            let st = harbor_core::orchestrator::status(PathBuf::from(state_path))?;
            for (name, pid, alive) in st {
                println!("{} {} {}", name, pid, if alive { "alive" } else { "dead" });
            }
            Ok(())
        }
        Commands::Logs {
            service,
            logs_dir,
            stream,
        } => {
            let path = match stream.as_str() {
                "stdout" => PathBuf::from(format!("{}/{}.out.log", logs_dir, service)),
                "stderr" => PathBuf::from(format!("{}/{}.err.log", logs_dir, service)),
                _ => PathBuf::from(format!("{}/{}.out.log", logs_dir, service)),
            };
            let content = std::fs::read_to_string(path)?;
            println!("{}", content);
            Ok(())
        }
        Commands::TrayInstall { source } => tray_install(source),
        Commands::TrayUninstall => tray_uninstall(),
    }
}

fn init_config(path: &str) -> Result<()> {
    let sample = r#"services:
  - name: web
    command: "node server.js"
    cwd: "."
    depends_on: []
    health_check:
      kind: http
      url: "http://localhost:3000/health"
      timeout_ms: 5000
      retries: 10
"#;
    std::fs::write(path, sample)?;
    println!("created {}", path);
    Ok(())
}

#[cfg(windows)]
fn tray_install(source: Option<String>) -> Result<()> {
    let src = if let Some(s) = source {
        PathBuf::from(s)
    } else {
        PathBuf::from("target/release/harbor-tray.exe")
    };
    if !src.exists() {
        anyhow::bail!("source not found: {}", src.display());
    }
    let install_dir = std::env::var("LOCALAPPDATA")
        .map(|p| PathBuf::from(p).join("Harbor"))
        .unwrap_or(PathBuf::from("C:\\Harbor"));
    std::fs::create_dir_all(&install_dir)?;
    let dest = install_dir.join("harbor-tray.exe");
    std::fs::copy(&src, &dest)?;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", winreg::enums::KEY_WRITE)?;
    let val = format!("\"{}\"", dest.display());
    path.set_value("HarborTray", &val)?;
    println!("installed {}", dest.display());
    Ok(())
}

#[cfg(not(windows))]
fn tray_install(_source: Option<String>) -> Result<()> {
    anyhow::bail!("windows only");
}

#[cfg(windows)]
fn tray_uninstall() -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(path) = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", winreg::enums::KEY_WRITE) {
        let _ = path.delete_value("HarborTray");
    }
    println!("uninstalled");
    Ok(())
}

#[cfg(not(windows))]
fn tray_uninstall() -> Result<()> {
    anyhow::bail!("windows only");
}

fn init_downloads_config(path: &str) -> Result<()> {
    let sample = r#"download_dir: "C:\\Users\\%USERNAME%\\Downloads"
min_age_secs: 5
rules:
  - name: images
    extensions: ["jpg", "jpeg", "png", "gif", "webp"]
    target_dir: "C:\\Users\\%USERNAME%\\Downloads\\Images"
  - name: videos
    extensions: ["mp4", "mov", "mkv", "avi"]
    target_dir: "C:\\Users\\%USERNAME%\\Downloads\\Videos"
  - name: archives
    extensions: ["zip", "rar", "7z", "tar", "gz"]
    target_dir: "C:\\Users\\%USERNAME%\\Downloads\\Archives"
  - name: docs
    extensions: ["pdf", "docx", "xlsx", "pptx", "txt"]
    target_dir: "C:\\Users\\%USERNAME%\\Downloads\\Documents"
  - name: installers
    extensions: ["exe", "msi"]
    target_dir: "C:\\Users\\%USERNAME%\\Downloads\\Installers"
"#;
    std::fs::write(path, sample)?;
    println!("created {}", path);
    Ok(())
}
