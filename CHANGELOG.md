## 📝 Changelog — Version `v0.4.3` (2025-10-21)

### ✨ New Features

* 🎯 **Automatic Games/ Path Prefix**
  Simplified path management for fake processes.

  * **Automatic prefix** - Just write `Fortnite/Win64` instead of `Games/Fortnite/Win64`.
  * **Visual preview** - Shows full path `📁 Games/...` before creating process.
  * **Backward compatible** - Still accepts full paths with `Games/` prefix.
  * **Updated presets** - All official presets now use simplified paths.

* 🔄 **Automatic Rich Presence Reset**
  Rich Presence now automatically updates when fake processes end.

  * **Process monitoring** - Tracks all running fake processes.
  * **Auto-reset to "Waiting..."** - Rich Presence clears when process closes.
  * **No manual intervention** - Everything happens automatically in background.
  * **Optimized checks** - Verifies process status every 2 seconds.

---

### 🛠️ Improvements

* **Path Management**: Simplified user experience with automatic `Games/` prefix
  * Users only need to specify relative paths from Games folder
  * Full path displayed as visual confirmation before execution
  * Cleaner preset definitions without repetitive `Games/` prefix

* **Rich Presence Lifecycle**: Complete automation of Discord presence updates
  * Intelligent monitoring of spawned processes
  * Automatic cleanup when processes terminate
  * Better synchronization between app state and Discord status

---

### 🐞 Fixes

* **Rich Presence Persistence**: Fixed issue where Rich Presence wouldn't reset after closing fake process
  * Implemented `ProcessMonitor` system to track active processes
  * Added periodic checks to detect terminated processes
  * Rich Presence now correctly returns to "Esperando..." state

---

### 🔧 Technical Changes

* Added `ProcessMonitor` struct for managing fake process lifecycles
* Implemented PID tracking system using `sysinfo` crate
* Enhanced `create_fake_process()` to return process ID
* Added `check_dead_processes()` method with 2-second check interval
* Updated all presets to use simplified path format
* Modified `process.rs` to automatically prepend `Games/` to relative paths

---

## 📝 Changelog — Version `v0.4.2` (2025-10-11)

### ✨ New Features

* 🎯 **Custom Preset Management**
  Complete system for managing custom presets directly from the interface.

  * **Add custom presets** without editing JSON files manually.
  * **Edit existing presets** - modify name, executable, or path.
  * **Delete custom presets** with confirmation dialog.
  * **Visual distinction** - custom presets marked with ⭐ icon.
  * **Separate storage** - custom presets stored in `presets_custom.json`.

* 🔒 **Safe Preset Updates**
  GitHub preset updates now preserve user-created presets.

  * **Dual file system** - official presets in `presets.json`, custom in `presets_custom.json`.
  * **Protected custom presets** - updating from GitHub only affects official presets.
  * **No data loss** - your custom presets remain intact during updates.

---

### 🛠️ Improvements

* **Performance Optimization**: Massive UI performance improvements in Main tab
  * **Discord detection cache** - checks only every 5 seconds instead of every frame.
  * **Reduced animations** - faster, smoother interface response.
  * **Eliminated stuttering** - window dragging and interaction now buttery smooth.
  * **Smart invalidation** - cache updates immediately when needed (e.g., opening Discord).

* **Better UX**: Improved preset management workflow
  * **Intuitive dialogs** - clear forms for adding/editing presets.
  * **Duplicate prevention** - validates preset names before saving.
  * **Confirmation dialogs** - prevents accidental deletion.
  * **Error handling** - clear feedback for invalid inputs.

---

### 🌐 Translations

* Added new translation keys for preset management:
  * `"add_preset"`, `"add_preset_title"`, `"edit_preset"`, `"edit_preset_title"`
  * `"delete_preset"`, `"delete_preset_title"`, `"delete_preset_confirm"`
  * `"preset_name"`, `"save_preset"`, `"delete"`, `"cancel"`
  * `"preset_fields_empty"`, `"preset_added_success"`, `"preset_edited_success"`, `"preset_deleted_success"`

---

### 🐞 Fixes

* **Performance Issues**: Fixed severe performance problems in Main tab
  * Eliminated constant Discord process checks causing UI stuttering
  * Reduced unnecessary system calls during rendering
  * Fixed laggy interactions with buttons and text fields

* **Preset Management**: Improved reliability
  * Better error messages for invalid preset data
  * Fixed potential race conditions when saving presets
  * Improved file handling for custom presets

---

### 🔧 Technical Changes

* Added `presets_custom.json` for user-created presets
* Implemented caching system with `std::time::Instant` for Discord checks
* Enhanced `Preset` struct with `is_custom` field
* New preset management functions: `add_preset()`, `edit_custom_preset()`, `delete_custom_preset()`
* Optimized egui animation timing for better responsiveness
* Added cache invalidation system for immediate updates when needed

---

## 📝 Changelog — Version `v0.4.1` (2025-09-20)

### 🛠️ Improvements

* **Code Architecture**: Complete UI modularization for better maintainability
  * Split `src/app/ui.rs` into separate modules: `main_tab.rs`, `settings_tab.rs`, `about_tab.rs`
  * Added `components.rs` module for reusable UI components
  * **Improved code organization** with focused, single-responsibility modules
  * **Better scalability** - easier to add new tabs and components
  * **Enhanced maintainability** - smaller, more focused files

---

### 🔧 Technical Changes

* Reorganized UI structure from monolithic file to modular architecture:
  ```
  src/app/ui/
  ├── main_tab.rs      - Main tab logic and components
  ├── settings_tab.rs  - Settings tab logic and components  
  ├── about_tab.rs     - About tab logic and components
  └── components.rs    - Reusable UI components
  ```
* **No breaking changes** - all functionality remains identical
* **Improved developer experience** with cleaner, more navigable codebase

---

## 📝 Changelog — Version `v0.4.0` (2025-09-17)

### ✨ New Features

* 🎮 **Discord Rich Presence Integration**
  DSQProcess now displays your simulated game activity on Discord!

  * Shows **"Playing [Game Name]"** on your Discord profile instead of executable names.
  * Displays **elapsed time** since DSQProcess started.
  * **Toggle option** in Settings to enable/disable Rich Presence (enabled by default).
  * Smart **game name detection** - shows "Borderlands 4" instead of "Borderlands4.exe".

* 🔍 **Manual Preset Verification**
  Check for preset updates without restarting the application.

  * Added **"Check Presets"** button in the Main tab.
  * Instant verification of local vs. remote preset files.
  * **Status confirmation** when presets are up-to-date.

* 🎨 **Enhanced User Interface**
  Completely redesigned interface with improved organization and centering.

  * **Centered elements** - tabs, buttons, and controls now properly aligned.
  * **Grouped sections** with visual cards for better organization.
  * **Consistent spacing** and professional layout throughout.
  * **Icons added** to buttons and sections for better visual feedback.

---

### 🛠️ Improvements

* **Code Architecture**: Rich Presence system organized in dedicated modules (`src/shared/richpresence/`).
* **UI Performance**: Optimized rendering with better widget management.
* **Error Handling**: Improved error messages and status feedback for Rich Presence.
* **Settings Persistence**: Rich Presence preferences saved in configuration.
* **Library Updates**: Updated to `discord-rich-presence v1.0.0` for better stability.

---

### 🌐 Translations

* Added new translation keys for Rich Presence and UI improvements:
  * `"enable_rich_presence"`, `"rich_presence_connected"`, `"rich_presence_disconnected"`
  * `"rich_presence_error"`, `"check_presets"`, `"presets_up_to_date"`

---

### 🐞 Fixes

* **Centering Issues**: Fixed misaligned tabs, combo boxes, and buttons.
* **Borrow Checker**: Resolved Rust compilation errors related to mutable/immutable borrowing.
* **Preset Detection**: Improved game name extraction from presets for Rich Presence.
* **Memory Management**: Better cleanup of Rich Presence connections on app close.

---

### 🔧 Technical Changes

* Updated `Cargo.toml` dependencies to include `discord-rich-presence = "1.0.0"`.
* Enhanced `Config` struct to include `rich_presence_enabled` field.
* Improved application state management with `RichPresenceManager`.
* Better error handling for Discord IPC connection failures.

---

## 📝 Changelog — Version `v0.3.0` (2025-07-03)

### ✨ New Features

* 🔁 **Remote Presets Update**
Presets can now be kept up-to-date by automatically comparing your local file with the one in the official repository.

  * Added a button to **update the `presets.json` file** directly from GitHub.
  * A warning is displayed if your local file is out of date.

* 🛡️ **Discord Running Detection**
DSQProcess now checks if **Discord, Discord PTB, or Canary** are open.

  * If they are not active, a warning message is displayed.
  * If versions are installed, buttons are provided to open them directly from the app.

* 🧭 **Tabbed Interface**
The main UI has been reorganized with a tabbed system:

  * **Main:** Fake process configuration.
  * **Settings:** Language switcher and future settings.
  * **About:** Current version, update checks, and credits.

---

### 🛠️ Improvements

* Code reorganized into **modules and subcomponents** for easier maintenance.
* The app **remembers the last configuration** thanks to the persistence system (`config.json`).
* Text restructured to support translations in tabs and new messages.

---

### 🌐 Translations

* Added new keys to the multilingual translation system:

  * `"discord_not_running"`, `"start_discord_prompt"`, `"discord_not_installed"`, `"tab_main"`, `"tab_settings"`, `"tab_about"`

---

### 🐞 Fixes

* Fixed a bug where the tab would always return to "Main" when trying to switch.
* Improved stability when comparing local and remote files in the preset verification.

---
