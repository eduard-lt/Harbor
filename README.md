⚓ Harbor

Harbor is a high-performance, cross-platform file organizer built in Rust. It acts as an automated "dock manager" for your Downloads folder, intelligently routing incoming files to their designated "warehouses" based on file type, metadata, and custom rules.
🚀 The Vision

Most download folders are a "graveyard" of forgotten files. Harbor stays in the background, watches for new arrivals, and instantly moves them to organized sub-folders (e.g., /Videos, /Archives, /Torrents) before you even have a chance to lose track of them.
Key Features

    Intelligent Sorting: Uses MIME type "magic numbers," not just extensions.

    Cross-Platform: Native support for Windows, macOS, and Linux.

    Safety First: Detects partial downloads (Chrome/Firefox/Safari) to prevent moving files before they are finished.

    Blazing Fast: Written in Rust for minimal CPU/RAM footprint.

🗺️ Development Roadmap
Phase 1: The Foundation (The Watcher)

Goal: Successfully detect when a file is added to the Downloads folder.

Initialize Rust project with notify crate.

Use directories crate to dynamically find the OS-specific Download path.

Implement a basic event loop that prints "File Detected" to the console.

    Handle the "Temporary File" problem: Ignore .crdownload, .part, and .tmp files.

Phase 2: The Brain (Classification)

Goal: Correctily identify what a file is.

Integrate mime_guess or infer to identify files by content.

Create a classification engine for:

    Media: .mp4, .mkv, .mov, .png, .jpg

    Archives: .zip, .rar, .7z, .tar.gz

    Executables: .exe, .msi, .dmg, .appimage

    Documents: .pdf, .docx, .xlsx

    Define the "Warehouse" structure (the sub-folders where files will live).

Phase 3: The Tugboat (File Operations)

Goal: Safely move files without data loss.

Implement the move logic.

Collision Handling: If image.jpg already exists in the destination, rename the new one to image_1.jpg.

    Debouncing: Implement a "settle timer." Wait for 2 seconds after the last "write" event to ensure the browser has fully released the file.

Phase 4: Persistence & Config (The Logbook)

Goal: Allow users to customize where files go.

Create a harbor.toml configuration file.

Allow users to map specific extensions to custom folders (e.g., .torrent -> C:/MyTorrents).

    Add a logging system to keep track of what was moved and where.

Phase 5: Stealth Mode (System Tray)

Goal: Run Harbor in the background without a terminal window.

Integrate tray-icon or Tauri for a system tray menu.

Add "Pause/Resume" functionality.

Add "Open Harbor Folder" and "Run Now" buttons.

    Set up "Launch on Startup" for Windows/Mac/Linux.

Phase 6: Distribution (Shipping)

Set up GitHub Actions to automatically compile .exe, .dmg, and Linux binaries.

    Optimize the binary size (LTO and stripping symbols).

🛠️ Tech Stack

    Language: Rust 🦀

    Event Monitoring: notify

    File Identification: infer / mime_guess

    Path Management: directories

    Background/UI: Tauri (for Phase 5+)

🛠 Installation (For Developers)

    Clone the repository
    code Bash

    
git clone https://github.com/yourusername/harbor.git
cd harbor

  

Run in Debug Mode
code Bash

    
cargo run

  

Build for Release
code Bash

        
    cargo build --release

      

📄 License

MIT - Created by Eduard Olteanu
Why Rust for this?

    Safety: Moving files is risky. Rust ensures we don't have "race conditions" where two threads try to move the same file at once.

    Speed: Harbor needs to be invisible. Rust uses almost zero CPU while waiting for files.

    Single Binary: You can send the finished .exe to a friend, and they don't need to install Python or a Runtime. It just works.
