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
