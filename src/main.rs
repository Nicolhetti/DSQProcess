#![windows_subsystem = "windows"]

use eframe::egui;
use serde::{ Serialize, Deserialize };
use std::collections::HashMap;

type LangMap = HashMap<String, String>;

const VERSION: &str = "0.1.0";

#[derive(Serialize, Deserialize, Default)]
struct Config {
    language: String,
    selected_preset: usize,
    process_name: String,
    custom_path: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Preset {
    name: String,
    executable: String,
    path: String,
}

#[derive(Default)]
struct DsqApp {
    process_name: String,
    custom_path: String,
    status: String,
    presets: Vec<Preset>,
    selected_preset: usize,
    langs: HashMap<String, LangMap>,
    selected_lang: String,
}

impl DsqApp {
    fn t(&self, key: &str) -> String {
        self.langs
            .get(&self.selected_lang)
            .and_then(|map| map.get(key))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
}

fn save_config(config: &Config) {
    let _ = std::fs::write("config.json", serde_json::to_string_pretty(config).unwrap_or_default());
}

fn load_config() -> Config {
    let data = std::fs::read_to_string("config.json").unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

fn load_language(code: &str) -> LangMap {
    let path = format!("lang/{}.json", code);
    let data = std::fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

fn load_presets() -> Vec<Preset> {
    let data = std::fs::read_to_string("presets.json").unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

fn check_for_updates(current_version: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/Nicolhetti/DSQProcess/releases/latest";
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).header("User-Agent", "dsqprocess").send()?;

    let json: serde_json::Value = response.json()?;
    let latest_version = json["tag_name"].as_str().unwrap_or("v0.0.0").trim_start_matches('v');
    let current = semver::Version::parse(current_version)?;
    let latest = semver::Version::parse(latest_version)?;

    if latest > current {
        Ok(Some(json["assets"][0]["browser_download_url"].as_str().unwrap_or("").to_string()))
    } else {
        Ok(None)
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([420.0, 400.0]),
        ..Default::default()
    };

    let config = load_config();
    let mut app = DsqApp::default();

    app.presets = load_presets();
    app.langs.insert("Español".to_string(), load_language("es"));
    app.langs.insert("English".to_string(), load_language("en"));

    app.selected_lang = config.language;
    app.selected_preset = config.selected_preset;
    app.process_name = config.process_name;
    app.custom_path = config.custom_path;

    eframe::run_native(
        "DSQProcess",
        options,
        Box::new(|_cc| Box::new(app))
    )
}

impl eframe::App for DsqApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.heading(self.t("title"));

                ui.horizontal(|ui| {
                    ui.label(self.t("lang"));
                    egui::ComboBox
                        ::from_id_source("language_select")
                        .selected_text(&self.selected_lang)
                        .show_ui(ui, |ui| {
                            for key in self.langs.keys() {
                                ui.selectable_value(&mut self.selected_lang, key.clone(), key);
                            }
                        });
                });

                if !self.presets.is_empty() {
                    egui::ComboBox
                        ::from_label(self.t("select_preset"))
                        .selected_text(&self.presets[self.selected_preset].name)
                        .show_ui(ui, |ui| {
                            for (i, preset) in self.presets.iter().enumerate() {
                                ui.selectable_value(&mut self.selected_preset, i, &preset.name);
                            }
                        });

                    if ui.button(self.t("use_preset")).clicked() {
                        let preset = &self.presets[self.selected_preset];
                        self.process_name = preset.executable.clone();
                        self.custom_path = preset.path.clone();
                    }

                    ui.separator();
                }

                ui.label(self.t("executable_name"));
                ui.text_edit_singleline(&mut self.process_name);

                ui.label(self.t("path_label"));
                ui.text_edit_singleline(&mut self.custom_path);

                if ui.button(self.t("start_process")).clicked() {
                    if !self.process_name.trim().is_empty() {
                        let result = create_fake_process(&self.custom_path, &self.process_name, 15);
                        self.status = match result {
                            Ok(_) =>
                                self
                                    .t("success")
                                    .replace("{name}", &self.process_name)
                                    .replace("{path}", &self.custom_path),
                            Err(e) => self.t("error").replace("{error}", &e.to_string()),
                        };
                    } else {
                        self.status = self.t("error_empty");
                    }
                }

                if ui.button(self.t("check_update")).clicked() {
                    match check_for_updates(VERSION) {
                        Ok(Some(download_url)) => {
                            self.status = self
                                .t("update_available")
                                .replace("{url}", &download_url);
                            let _ = open::that(download_url);
                        }
                        Ok(None) => {
                            self.status = self.t("up_to_date");
                        }
                        Err(e) => {
                            self.status = self.t("update_error").replace("{error}", &e.to_string());
                        }
                    }
                }

                ui.separator();
                ui.label(&self.status);

                let config = Config {
                    language: self.selected_lang.clone(),
                    selected_preset: self.selected_preset,
                    process_name: self.process_name.clone(),
                    custom_path: self.custom_path.clone(),
                };
                save_config(&config);
            });
        });
    }
}

fn create_fake_process(folder: &str, exe_name: &str, duration_min: u64) -> std::io::Result<()> {
    let target_folder = std::path::Path::new(folder);
    std::fs::create_dir_all(target_folder)?;

    let new_exe_path = target_folder.join(exe_name);

    let current_exe = std::env::current_exe()?;
    let child_path = current_exe
        .parent()
        .unwrap()
        .join(if cfg!(windows) { "dsqchild.exe" } else { "dsqchild" });

    std::fs::copy(&child_path, &new_exe_path)?;
    std::process::Command::new(&new_exe_path).arg(duration_min.to_string()).spawn()?;

    Ok(())
}
