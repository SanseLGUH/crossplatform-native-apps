use eframe::egui;
use smart_default::SmartDefault;

use crossbeam::atomic::AtomicCell;

use crate::{
	fetching::{metadata, install}, structs::{MiniGameCollection, MiniGame}
};

pub struct NazzyAuto {
    key: String,    
	current_mode: MiniGame,
    list_modes: MiniGameCollection
}

impl NazzyAuto {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let md = metadata().expect("Failed to fetch metadata");

        let last_key = md.iter().last().and_then(|(key, _)| Some(key.clone())).unwrap_or_else(|| {
            "".to_string()
        });

        Self {
            key: last_key.clone(),
            current_mode: md.get(&last_key).unwrap().clone(),
            list_modes: md
        }
    }

    fn update_mode(&mut self) {
        self.current_mode = self.list_modes.get(&self.key).unwrap().clone();
    }

    fn install(&self) {
    }
}

impl eframe::App for NazzyAuto {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update_mode();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_enabled(
                false,
                egui::TextEdit::multiline(&mut self.current_mode.path)
                    .font(egui::TextStyle::Monospace)
                    .desired_rows(1)
                    .desired_width(f32::INFINITY),
            );

            ui.add_enabled( false,
                egui::TextEdit::multiline(&mut self.current_mode.description)
                    // .text_color( self.logs.color )
                    .font(egui::TextStyle::Monospace)
                    .desired_rows(3)
                    .desired_width(f32::INFINITY)
            );

            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), 20.0),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    if ui.button("Установить").clicked() {
                    }

                    egui::ComboBox::from_label("")
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