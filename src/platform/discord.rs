use std::process::Command;
use std::path::{ Path, PathBuf };
use sysinfo::System;

#[derive(Debug, Clone, PartialEq)]
pub enum DiscordVersion {
    Stable,
    Canary,
    PTB,
}

impl DiscordVersion {
    pub fn exe_name(&self) -> &'static str {
        match self {
            DiscordVersion::Stable => "Discord.exe",
            DiscordVersion::Canary => "DiscordCanary.exe",
            DiscordVersion::PTB => "DiscordPTB.exe",
        }
    }

    pub fn folder_name(&self) -> &'static str {
        match self {
            DiscordVersion::Stable => "Discord",
            DiscordVersion::Canary => "DiscordCanary",
            DiscordVersion::PTB => "DiscordPTB",
        }
    }
}

pub fn is_discord_running() -> bool {
    let system = System::new_all();
    for process in system.processes_by_name("Discord") {
        if process.name().contains("Discord") {
            return true;
        }
    }
    for process in system.processes_by_name("DiscordCanary") {
        if process.name().contains("DiscordCanary") {
            return true;
        }
    }
    for process in system.processes_by_name("DiscordPTB") {
        if process.name().contains("DiscordPTB") {
            return true;
        }
    }
    false
}

pub fn get_installed_discord_versions() -> Vec<DiscordVersion> {
    let mut found = vec![];
    if let Some(local_appdata) = std::env::var_os("LOCALAPPDATA") {
        for version in &[DiscordVersion::Stable, DiscordVersion::Canary, DiscordVersion::PTB] {
            let path = Path::new(&local_appdata).join(version.folder_name()).join("Update.exe");
            if path.exists() {
                found.push(version.clone());
            }
        }
    }
    found
}

pub fn open_discord(version: DiscordVersion) -> std::io::Result<()> {
    if let Some(local_appdata) = std::env::var_os("LOCALAPPDATA") {
        let update_exe: PathBuf = Path::new(&local_appdata)
            .join(version.folder_name())
            .join("Update.exe");
        if update_exe.exists() {
            Command::new(update_exe).arg("--processStart").arg(version.exe_name()).spawn()?;
        }
    }
    Ok(())
}
