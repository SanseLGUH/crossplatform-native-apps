use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    
    eframe::run_native("Sansel's Calculator", native_options, 
        Box::new(|cc| 
            Ok(Box::new(Calculator::new(cc)))
        )
    );
}

#[derive(Default)]
struct Calculator {}

impl Calculator {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self::default()
    }
}


use egui::{Button, Vec2};

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();

            let button_width = available_size.x / 3.0 - 5.5; // padding adjustment
            let button_height = available_size.y / 4.0 + 20.0; // 3 rows + space for display

            let button = Button::new("display").min_size(Vec2::new(available_size.x , 50.));

            if ui.add_enabled(false, button).clicked() {
                unreachable!();
            }

            ui.vertical_centered(|ui| {
                for row in 0..3 {
                    ui.horizontal(|ui| {
                        for col in 0..3 {
                            let number = row * 3 + col + 1;
                            let button = Button::new(number.to_string())
                                .min_size(Vec2::new(button_width, button_height));
                            if ui.add(button).clicked() {
                                println!("Pressed {}", number);
                            }
                        }
                    });
                }
            });
        });
    }
}
