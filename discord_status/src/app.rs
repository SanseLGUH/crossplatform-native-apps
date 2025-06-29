use crate::core::*;
use eframe::egui;

use serde::Serialize;

#[derive(Serialize)]
pub struct Settings {
    datails: String,
    state: String,
    name: String,
    r#type: u64,
    url: String
}

#[derive(Default)]
pub struct DiscordActivityApp {
    token: String,
    working: Option<tokio::task::JoinHandle<()>>
}

// eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(app::MyEguiApp::new(cc)))));

impl DiscordActivityApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for DiscordActivityApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
        
            ui.vertical_centered(|ui| {
                egui::TextEdit::multiline(&mut self.token).hint_text("token").show(ui);
            });

            ui.vertical_centered(|ui| {
                if ui.add( egui::Button::new( if !self.working.is_some() { "START" } else { "STOP" } ).min_size(egui::Vec2::new(20., 20.)) ).clicked() {    
                    if let Some(task) = &self.working {
                        println!("aborting");
                        task.abort();
                        self.working = None;
                    }
                    
                    else {
                        self.working = Some(tokio::spawn(async move {
                            let conn = connect("test").await;
                            // Do something with conn
                        }));
                    }
               
                }
            });
        
        });
    }
}
