mod fetching;
mod ui;
mod structs;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.resizable = Some(false);
    native_options.viewport.maximize_button = Some(false);
    native_options.viewport.max_inner_size = Some( eframe::egui::Vec2::new(325., 150.) );
    eframe::run_native("Nazzy-auto - eframe edition", native_options, Box::new(|cc| Ok(Box::new( ui::NazzyAuto::new(cc))))).unwrap();
}
