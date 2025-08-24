mod file_manager;

mod app;

// mod backend;
mod client;

mod settings;
// mod error;
mod logs;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.resizable = Some(false);
    native_options.viewport.maximize_button = Some(false);
    native_options.viewport.max_inner_size = Some( eframe::egui::Vec2::new(300., 410.) );
    eframe::run_native("Discord Custom Activity", native_options, Box::new(|cc| Ok(Box::new(app::DiscordActivityApp::new(cc)))));
}