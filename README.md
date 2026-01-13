⚓ Harbor

Version: 0.3.0

Harbor is a lightweight Windows utility that keeps your Downloads folder tidy. It watches for stable files and moves them into organized folders based on extensions and simple patterns. A tray app provides quick control and feedback; a CLI helps with power‑user workflows.

Features

- Tray app with Start/Stop, Organize Now, Open Downloads/Config/Recent Moves
- Startup registration without admin (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`)
- Checkmarks on Start/Stop indicate current watching state
- Safe moves: ignores partial files (`.crdownload`, `.part`, `.tmp`, `.download`)
- Conflict-free renames: appends `(n)` when the destination exists
- Simple YAML config with `%ENV%` expansion like `%USERPROFILE%`
- Recent actions log under `%LOCALAPPDATA%\Harbor\recent_moves.log`
- **Smart Symlinks**: Optionally leave a shortcut behind so browsers don't "lose" the file.
  - Shortcuts are **hidden** to keep your folder clean.
  - **Auto-Cleanup**: Old shortcuts are automatically removed when Harbor restarts.

Quick Start

- Build tray and CLI:
  - `cargo build --release -p harbor-tray -p harbor-cli`
- Install tray for startup:
  - `target\release\harbor-cli.exe tray-install`
- Run tray now (no console window):
  - `"%LOCALAPPDATA%\Harbor\harbor-tray.exe"`

Tray Menu

- `Start Watching` / `Stop Watching`: toggles the background organizer
- `Organize Now`: runs a one-time pass immediately
- `Open Downloads`: opens the configured downloads directory
- `Open Config`: opens `%LOCALAPPDATA%\Harbor\harbor.downloads.yaml` if present
- `Open Recent Moves`: opens `%LOCALAPPDATA%\Harbor\recent_moves.log`
- `Exit`: closes the tray app

CLI

- `harbor-cli tray-install` — copy tray binary to `%LOCALAPPDATA%\Harbor` and register startup
- `harbor-cli tray-uninstall` — remove startup registry entry
- `harbor-cli downloads-init PATH` — write a starter downloads YAML (optional)
- `harbor-cli downloads-organize PATH` — run once and print moves
- `harbor-cli downloads-watch PATH --interval-secs 5` — watch via CLI

Configuration

- Location:
  - `%LOCALAPPDATA%\Harbor\harbor.downloads.yaml` is preferred
  - If missing, Harbor uses built‑in defaults and still runs
- Format:
  ```yaml
  download_dir: "%USERPROFILE%\\Downloads"
  min_age_secs: 5
  rules:
    - name: Images
      extensions: ["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "heic", "svg"]
      target_dir: "%USERPROFILE%\\Downloads\\Images"
      create_symlink: true
    - name: Videos
      extensions: ["mp4", "mkv", "avi", "mov", "wmv", "webm"]
      target_dir: "%USERPROFILE%\\Downloads\\Videos"
    - name: Music
      extensions: ["mp3", "flac", "wav", "aac", "ogg"]
      target_dir: "%USERPROFILE%\\Downloads\\Music"
    - name: Archives
      extensions: ["zip", "rar", "7z", "tar", "gz", "xz"]
      target_dir: "%USERPROFILE%\\Downloads\\Archives"
    - name: Installers
      extensions: ["exe", "msi", "msix", "dmg", "pkg", "apk"]
      target_dir: "%USERPROFILE%\\Downloads\\Installers"
    - name: Documents
      extensions: ["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "rtf"]
      target_dir: "%USERPROFILE%\\Downloads\\Documents"
  ```
- Rule options:
  - `extensions`: list of case‑insensitive extensions
  - `pattern`: optional regex applied to file name
  - `min_size_bytes` / `max_size_bytes`: optional size filters
  - `target_dir`: destination folder (supports `%ENV%` expansion)
  - `create_symlink`: optional boolean (default `false`). If `true`, leaves a symbolic link in the download folder pointing to the moved file.
    - **Hidden**: The link is marked as hidden to avoid clutter.
    - **Requirements**: Requires **Developer Mode** enabled in Windows Settings (or running as Admin).
  - `min_age_secs`: global stability delay before moving

Defaults

- If no config file is found, Harbor uses sensible defaults targeting subfolders inside `Downloads`:
  - `Downloads\Images`, `Videos`, `Music`, `Documents`
  - `Downloads\Archives`, `Installers`, `ISOs`, `Torrents`, `Webpages`, `Dev`, `Subtitles`

Startup

- Uses `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` (per‑user, no admin)
- Path is quoted to handle spaces
- Tray binary is built without a console window

Troubleshooting

- **Symlinks not created**:
  - Check `recent_moves.log` for "Symlink failed".
  - Ensure **Developer Mode** is on (Settings → Update & Security → For developers).
- Tray doesn’t start at login:
  - Remove old entries in Task Manager → Startup, then run `harbor-cli tray-install`
- Files not moving:
  - Check `min_age_secs`; large files may need more time to settle
  - Verify extensions and destination folders exist

Development

- Build: `cargo build --release -p harbor-tray -p harbor-cli`
- Tests: `cargo test -p harbor-core`
- Run CLI organize: `harbor-cli downloads-organize harbor.downloads.yaml`

License

MIT © Eduard Olteanu
