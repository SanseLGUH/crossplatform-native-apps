use eframe::egui;

#[derive(Default)]
pub struct DiscordStatusApp {}

// eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(app::MyEguiApp::new(cc)))));

impl DiscordStatusApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for DiscordStatusApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal_wrapped(|ui| {
                ui.label("some text");
                if ui.button("CREATE WINDOW").clicked() {

                    println!("hellow");
                }
            })
      });
    }
}
