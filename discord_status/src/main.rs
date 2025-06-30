mod app;
mod core;

#[tokio::main]
async fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.resizable = Some(false);
    native_options.viewport.maximize_button = Some(false);
    native_options.viewport.max_inner_size = Some( eframe::egui::Vec2::new(300., 340.) );
    eframe::run_native("Discord Custom Activity", native_options, Box::new(|cc| Ok(Box::new(app::DiscordActivityApp::new(cc)))));
}


