#![windows_subsystem = "windows"]

use eframe::{ egui, App, Frame };
use std::{ collections::HashMap, fs };

type LangMap = HashMap<String, String>;

#[derive(serde::Deserialize)]
struct Config {
    language: String,
}

fn load_language_from_config() -> LangMap {
    let config: Config = fs
        ::read_to_string("config.json")
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or(Config {
            language: "Español".to_string(),
        });

    let lang_code = if config.language == "English" { "en" } else { "es" };
    let path = format!("lang/{}_child.json", lang_code);
    let data = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

struct DsqChildApp {
    closing_time: Option<std::time::Instant>,
    status: String,
    lang: LangMap,
}

impl Default for DsqChildApp {
    fn default() -> Self {
        let lang = load_language_from_config();
        Self {
            closing_time: None,
            status: String::new(),
            lang,
        }
    }
}

impl DsqChildApp {
    fn t(&self, key: &str) -> String {
        self.lang
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
}

impl App for DsqChildApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.t("title"));

            if ui.button(self.t("close_in_15")).clicked() {
                self.closing_time = Some(
                    std::time::Instant::now() + std::time::Duration::from_secs(900)
                );
                self.status = self.t("scheduled_close");
            }

            ui.separator();

            if ui.button(self.t("close_now")).clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }

            if let Some(when) = self.closing_time {
                let now = std::time::Instant::now();
                if now >= when {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                } else {
                    let remaining = when.duration_since(now);
                    self.status = self
                        .t("auto_closing")
                        .replace("{min}", &format!("{:02}", remaining.as_secs() / 60))
                        .replace("{sec}", &format!("{:02}", remaining.as_secs() % 60));
                }
            }

            ui.separator();
            ui.label(&self.status);
        });

        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder
            ::default()
            .with_inner_size([300.0, 120.0])
            .with_resizable(false)
            .with_decorations(true),
        ..Default::default()
    };

    eframe::run_native(
        "Proceso falso",
        options,
        Box::new(|_cc| Box::new(DsqChildApp::default()))
    )
}
