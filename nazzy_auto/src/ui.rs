use eframe::egui;
use smart_default::SmartDefault;

use crossbeam::atomic::AtomicCell;

use std::{
    path::PathBuf, 
    sync::{
        Arc, atomic::{AtomicU32, AtomicBool, Ordering}
    }
};

use crate::{
	MiniGameCollection, MiniGame, fetching
};

fn default_minecraft_path() -> String {
    let default_path = dirs::config_dir().unwrap().join(".minecraft");

    match default_path.exists() {
        true => default_path.display().to_string(),
        false => String::from("переместите папку с Майнкрафтом в приложение!")
    }
}

pub struct NazzyAuto {
    is_downloading: Arc<AtomicBool>,
    progress: Arc<AtomicU32>,
    install_path: String,
    key: String,
	current_mode: MiniGame,
    list_modes: MiniGameCollection
}

impl NazzyAuto {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let md = fetching::metadata().expect("Failed to fetch metadata");

        let last_key = md.iter().last().and_then(|(key, _)| Some(key.clone())).unwrap_or_else(|| {
            "".to_string()
        });

        Self {
            is_downloading: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(AtomicU32::new(0)),
            install_path: default_minecraft_path(),
            key: last_key.clone(),
            current_mode: md.get(&last_key).unwrap().clone(),
            list_modes: md
        }
    }

    fn update_mode(&mut self) {
        if let Some(new_mode) = self.list_modes.get(&self.key) {
            self.current_mode = new_mode.clone();
        }
    }

    fn install(&self) {
        fetching::install(
            self.is_downloading.clone(), self.progress.clone(), 
            self.install_path.clone(), self.current_mode.path.clone()
        );
    }
}

impl eframe::App for NazzyAuto {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let progress = self.progress.load(Ordering::Relaxed) as f32 / 100.;
        let downloading = self.is_downloading.load(Ordering::Relaxed);

        self.update_mode();

        let files = ctx.input(|i| i.raw.hovered_files.clone());
        
        if !files.is_empty() {
            for file in &files {
                if let Some(path) = &file.path {
                    self.install_path = path.display().to_string().clone();
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_enabled(
                false,
                egui::TextEdit::multiline(&mut format!("{:?}", self.install_path))
                    .desired_rows(1)
                    .desired_width(f32::INFINITY),
            );

            ui.add(
                egui::ProgressBar::new(progress)
                    .fill(if progress == 1.0 {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::DARK_RED
                    })
            );

            ui.add_enabled(
                false,
                egui::TextEdit::multiline(&mut self.current_mode.path)
                    .font(egui::TextStyle::Monospace)
                    .desired_rows(1)
                    .desired_width(f32::INFINITY),
            );

            ui.add_enabled( false,
                egui::TextEdit::multiline(&mut self.current_mode.description)
                    .font(egui::TextStyle::Monospace)
                    .desired_rows(3)
                    .desired_width(f32::INFINITY)
            );

            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), 20.0),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    if ui.add_enabled( !downloading && PathBuf::from(self.install_path.clone()).exists(), egui::Button::new("Установить") ).clicked() {
                        self.install();
                    }

                    if ui.add_enabled( false, egui::Button::new("Отмена") ).clicked() {
                    }

                    if ui.add_enabled( false, egui::Button::new("Очистить") ).clicked() {
                    }

                    egui::ComboBox::from_label("")
                    .height(4.)
                    .selected_text(&self.key)
                    .show_ui(ui, |ui| {
                        for key in self.list_modes.keys() {
                            ui.selectable_value(&mut self.key, key.clone(), key);
                        }
                    });
                },
            );
		});
	}
}