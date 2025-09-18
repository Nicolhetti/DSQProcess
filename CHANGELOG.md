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
