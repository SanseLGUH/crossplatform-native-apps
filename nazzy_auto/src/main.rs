mod fetching;
mod ui;

use serde::{Serialize, Deserialize};

pub const METADATA: &str = "http://178.250.187.252:4462/manifest/na_meta.json";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MiniGame {
    pub path: String,
    pub description: String
}

pub type MiniGameCollection = std::collections::HashMap<String, MiniGame>;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.resizable = Some(true);
    native_options.viewport.maximize_button = Some(false);

    native_options.viewport.inner_size = Some( eframe::egui::Vec2::new(455., 160.) );
    native_options.viewport.min_inner_size = Some( eframe::egui::Vec2::new(440., 160.) );

    eframe::run_native("Nazzy-auto - eframe edition", native_options, Box::new(|cc| Ok(Box::new( ui::NazzyAuto::new(cc))))).unwrap();
}
