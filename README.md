<div align="center">

# ⚓ Harbor

**Keep your Downloads folder organized, automatically.**

<img src="assets/harbor_h.png" alt="Harbor Logo" width="400">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Windows](https://img.shields.io/badge/platform-Windows-blue.svg)](https://github.com/eduard-lt/Harbor/releases)
[![GitHub release](https://img.shields.io/github/v/release/eduard-lt/Harbor)](https://github.com/eduard-lt/Harbor/releases/latest)

[Features](#-features) •
[Installation](#-installation) •
[Configuration](#️-configuration) •
[Building](#-building-from-source) •
[Support](#-support)

</div>

---

## 🎯 What is Harbor?

Harbor is a lightweight Windows utility that automatically organizes your Downloads folder. No more manually sorting files - Harbor watches your downloads and moves them to the right folders based on file type, keeping everything tidy without you lifting a finger.

### How It Works

1. **Monitors Your Downloads** - Runs quietly in your system tray
2. **Detects New Files** - Waits for files to finish downloading (no partial files!)
3. **Organizes Automatically** - Moves files to categorized folders based on extension
4. **Stays Out of Your Way** - Minimal UI, maximum efficiency

Perfect for anyone tired of digging through a cluttered Downloads folder!

---

## ✨ Features

- **🔄 Auto-Organization** - Automatically sorts downloads by file type (images, videos, documents, etc.)
- **🎛️ Simple Tray Interface** - Start/Stop watching, organize now, or access recent activity
- **⚡ No Admin Required** - Installs and runs with user permissions only
- **🔗 Smart Symlinks** - Optionally leave hidden shortcuts so your browser doesn't "lose" files
- **📝 Activity Log** - Track what was moved and when in `recent_moves.log`
- **⚙️ Customizable Rules** - Edit the YAML config to create your own organization rules
- **🚀 Auto-Start** - Launches automatically on Windows startup
- **💾 Safe Moves** - Avoids partial downloads (.crdownload, .part, .tmp)
- **🔄 Conflict Handling** - Automatically renames files if destination already exists

---

## 📦 Installation

### Option 1: MSI Installer (Recommended)

1. Download the latest `.msi` installer from the [Releases page](https://github.com/eduard-lt/Harbor/releases)
2. Run `harbor-tray-x.x.x-x86_64.msi`
3. Harbor automatically starts and adds itself to your system tray
4. That's it! Your downloads will now be organized automatically

### Option 2: Portable Executables

1. Download `harbor-tray.exe` and `harbor-cli.exe` from [Releases](https://github.com/eduard-lt/Harbor/releases)
2. Place them in a folder of your choice
3. Run `harbor-tray.exe` to start the tray application
4. (Optional) Use `harbor-cli.exe` for command-line operations

### What Gets Installed?

Files are installed to `%LOCALAPPDATA%\Harbor\`:
- `harbor-tray.exe` - System tray application
- `harbor-cli.exe` - Command-line interface
- `harbor.downloads.yaml.default` - Template configuration
- `harbor.downloads.yaml` - Your active configuration (auto-created)
- Icons and activity logs

---

## 🎮 Using Harbor

### System Tray Menu

Right-click the Harbor icon in your system tray:

- **Start/Stop Watching** - Toggle automatic organization
- **Organize Now** - Manually organize downloads immediately  
- **Open Downloads** - Open your Downloads folder
- **Open Config** - Edit your organization rules
- **Open Recent Moves** - View the activity log
- **Exit** - Close Harbor

### Command Line Interface

Harbor also includes a CLI for power users:

```powershell
# Organize downloads once
harbor-cli downloads-organize

# Watch for new downloads (runs continuously)
harbor-cli downloads-watch --interval-secs 5

# Install tray app to startup
harbor-cli tray-install

# Remove from startup
harbor-cli tray-uninstall
```

---

## ⚙️ Configuration

Harbor uses a simple YAML configuration file located at:
```
%LOCALAPPDATA%\Harbor\harbor.downloads.yaml
```

### Default Organization Rules

By default, Harbor organizes files into these categories:

| Category | File Types | Destination |
|----------|-----------|-------------|
| 📸 Images | jpg, png, gif, webp, svg, etc. | `Downloads\Images` |
| 🎬 Videos | mp4, mkv, avi, mov, webm | `Downloads\Videos` |
| 🎵 Music | mp3, flac, wav, aac, ogg | `Downloads\Music` |
| 📄 Documents | pdf, docx, xlsx, txt, etc. | `Downloads\Documents` |
| 📦 Archives | zip, rar, 7z, tar, gz | `Downloads\Archives` |
| ⚙️ Installers | exe, msi, apk, dmg | `Downloads\Installers` |
| 💿 ISOs | iso | `Downloads\ISOs` |
| 🧲 Torrents | torrent | `Downloads\Torrents` |
| 🌐 Web Pages | html, htm | `Downloads\Webpages` |
| 💻 Dev Files | json, xml, env | `Downloads\Dev` |
| 📊 Subtitles | srt | `Downloads\Subtitles` |

### Customizing Rules

Edit `harbor.downloads.yaml` to customize your organization:

```yaml
download_dir: "%USERPROFILE%\\Downloads"
min_age_secs: 5  # Wait 5 seconds before organizing (ensures download is complete)

rules:
  - name: Screenshots
    extensions: ["png", "jpg"]
    pattern: "^(Screenshot|screen)"  # Regex pattern for filenames
    target_dir: "%USERPROFILE%\\Pictures\\Screenshots"
    create_symlink: false  # Set to true to leave a shortcut behind
    
  - name: Work Documents
    extensions: ["pdf", "docx"]
    pattern: "work|invoice|contract"
    min_size_bytes: 1024  # Only files larger than 1KB
    target_dir: "%USERPROFILE%\\Documents\\Work"
```

**Configuration Options:**
- `extensions` - List of file extensions to match
- `pattern` - Optional regex pattern for filename matching
- `min_size_bytes` / `max_size_bytes` - Optional size filters
- `target_dir` - Destination folder (supports `%USERPROFILE%`, `%USERNAME%`, etc.)
- `create_symlink` - Leave a hidden shortcut in Downloads (requires Developer Mode)

---

## 🛠️ Building from Source

Want to build Harbor yourself? Here's how!

### Prerequisites

1. **Install Rust** (latest stable)
   ```powershell
   # Visit https://rustup.rs/ or run:
   winget install Rustlang.Rustup
   ```

2. **Install WiX Toolset v3** (for MSI installer)
   ```powershell
   winget install WiXToolset.WiXToolset
   ```

3. **Install cargo-wix**
   ```powershell
   cargo install cargo-wix
   ```

### Build Steps

#### Prerequisites

1.  **Node.js & npm** (Tested with Node 20+)
2.  **Rust** (Latest stable)
3.  **WiX Toolset v3** (for MSI installer)

#### Build Order

Harbor is a Tauri v2 app (React Frontend + Rust Backend).

```powershell
# 1. Install Frontend Dependencies
cd packages/ui
npm install

# 2. Build App (Frontend + Backend + Installer)
npm run tauri:build

# Installer Location:
# target/release/bundle/msi/Harbor_1.0.0_x64_en-US.msi
```

### Local Testing

```powershell
# Run in Development Mode (Hot Reload)
cd packages/ui
npm run tauri:dev
```

---

## 🔧 Troubleshooting

### Symlinks Not Working

**Problem:** Files are moved but symlinks aren't created

**Solution:**
1. Enable **Developer Mode** in Windows:
   - Settings → System → For developers → Developer Mode: ON
2. Or run Harbor as Administrator (not recommended)
3. Check `recent_moves.log` for symlink errors

### App Doesn't Start at Login

**Problem:** Harbor doesn't auto-start after reboot

**Solution:**
1. Check Task Manager → Startup tab
2. Verify registry entry exists:
   ```
   HKCU\Software\Microsoft\Windows\CurrentVersion\Run\Harbor
   ```

### Files Not Being Organized

**Problem:** Downloaded files aren't being moved

**Checklist:**
- ✅ Is Harbor running? (Check system tray)
- ✅ Is watching enabled? (Right-click tray icon → Service On)
- ✅ Check `min_age_secs` - large files need time to fully download
- ✅ Verify file extension matches a rule in your config
- ✅ Ensure destination folders exist or can be created

### Multiple Instances Error

**Problem:** "Another instance of Harbor is already running"

**Solution:**
1. The app prevents multiple instances. It should focus the existing window.
2. If stuck, check Task Manager for `Harbor.exe` and end it.

---

## 📝 License

Harbor is released under the **MIT License**. See [LICENSE](LICENSE) for details.

This means you're free to:
- ✅ Use Harbor commercially
- ✅ Modify the source code
- ✅ Distribute copies
- ✅ Use it privately

The only requirement is including the original copyright notice.

---

## 🤝 Contributing

Contributions are welcome! Whether it's:
- 🐛 Bug reports
- 💡 Feature suggestions  
- 📖 Documentation improvements
- 🔧 Code contributions

Please feel free to open an issue or submit a pull request!

---

## 🗺️ Roadmap

### In Progress
- [ ] Cross-platform support (Linux, macOS)
- [x] GUI for editing rules without editing YAML (Settings Page)
- [x] Log viewer with search and filters (Activity Page)
- [ ] File organization preview before moving

### Planned Features
- [ ] Rule templates for common scenarios
- [ ] Scheduled organization times
- [ ] Network drive support
- [ ] Multiple download folder monitoring
- [ ] Custom notification settings

---

## 💬 Support

Need help or have questions?

- 📖 Check the [Documentation](#-configuration)
- 🐛 Report bugs via [GitHub Issues](https://github.com/eduard-lt/Harbor/issues)
- 💡 Request features via [GitHub Issues](https://github.com/eduard-lt/Harbor/issues)
- ⭐ Star the project if you find it useful!

---

## ☕ Support Development

If Harbor has made your life easier, consider buying me a coffee! Your support helps maintain and improve the project.

<div align="center">

[![Buy Me A Coffee](https://shields.io/badge/kofi-Buy_a_coffee-ff5f5f?logo=ko-fi&style=for-the-badge)](https://ko-fi.com/eduardolteanu)

*Every coffee helps keep Harbor sailing! ⚓*

</div>

---

<div align="center">

Made with ❤️ by Eduard Olteanu

[⬆ Back to Top](#-harbor)

</div>
