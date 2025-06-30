use crate::core::*;
use eframe::egui;

use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Settings {
    datails: String,
    state: String,
    name: String,
    r#type: u64,
    url: String
}

use tokio::{ task::JoinHandle };
use std::sync::Arc;
use crossbeam::atomic::AtomicCell;
#[derive(Default)]
pub struct WebsocketBackend {
    task: Option<JoinHandle<()>>, 
    connection_state: Arc< AtomicCell<ConnectionState> > ,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::Disconnected
    }
}

#[derive(Default)]
pub struct DiscordActivityApp {
    token: String,
    websocket_backend: WebsocketBackend,
    settings: Settings,
    offline_mode: bool
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let conn_state = self.websocket_backend.connection_state.load();

            ui.vertical_centered(|ui| {
                ui.heading("🎮 Discord Custom Activity");
                ui.label("Configure and run your custom Discord rich presence.");
                ui.add_space(10.0);

                ui.separator();

                // Settings group
                ui.group(|ui| {
                    ui.label("📋 Activity Settings");
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.add(egui::TextEdit::singleline(&mut self.settings.name).hint_text("Game / App Name").desired_width(200.0));
                    });

                    ui.horizontal(|ui| {
                        ui.label("State:");
                        ui.add(egui::TextEdit::singleline(&mut self.settings.state).hint_text("Status or detail").desired_width(200.0));
                    });

                    ui.horizontal(|ui| {
                        ui.label("URL:");
                        ui.add(egui::TextEdit::singleline(&mut self.settings.url).hint_text("https://...").desired_width(200.0));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Type:");
                        ui.add(egui::DragValue::new(&mut self.settings.r#type).clamp_range(0..=5).speed(1.0));
                        ui.label("(0: Playing, 1: Streaming, etc.)").on_hover_text("Refer to Discord activity types");
                    });

                    ui.horizontal(|ui| {
                        ui.label("🖼 Icon:");
                        ui.label("Drag and drop an image into the app");
                    });
                });

                ui.separator();

                // Token + Mode
                ui.group(|ui| {
                    ui.label("🔐 Discord Token");
                    ui.add_space(5.0);
                    ui.add_enabled_ui(!self.offline_mode, |ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.token)
                                .hint_text("Paste your token here")
                                .desired_width(300.0)
                                .background_color( if conn_state == ConnectionState::Failed { egui::Color32::LIGHT_RED } else { egui::Color32::from_gray(10) }  )
                        );
                    });
                });

                ui.add_space(10.0);

                // Mode toggle and start/stop
                ui.group(|ui| {
                    let btn_label = if conn_state == ConnectionState::Connected { "⏹ Stop" } else { "▶ Start" };
                    let button = egui::Button::new(btn_label).min_size(egui::Vec2::new(65.0, 15.0));
                    
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.offline_mode, true, "Offline Mode");
                        ui.selectable_value(&mut self.offline_mode, false, "WebSocket Mode");
                        
                        if ui.add(button).clicked() {
                            if !self.offline_mode {
                                match conn_state {
                                    ConnectionState::Connected => {},
                                    ConnectionState::Disconnected => {
                                        let token = self.token.clone();
                                        let arc_conn_state = self.websocket_backend.connection_state.clone();
                                        tokio::task::spawn( async move {
                                            println!("test");
                                            match connect(&token).await {
                                                Ok(_) => { arc_conn_state.store( ConnectionState::Connected ) },
                                                Err(_) => { arc_conn_state.store( ConnectionState::Failed ) }
                                            }
                                        });
                                    },
                                    _ => {}
                                }
                            } 

                            // TODO: handle start/stop logic
                        }
                    });
               });
            });
        });
    }
}

