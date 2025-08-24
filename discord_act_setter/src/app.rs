use eframe::egui;

use tokio::{ 
    task::{ self, JoinHandle }, 
    sync::oneshot 
};

use serde_json::to_string;

use crate::{
    client::{ SyncClient, 
        websocket::{ 
            structures::GatewayEvent, types::{AtomicState, WebSocketState} } 
        }, 
        logs, settings::*
}; 

#[derive(Default)]
pub struct DiscordActivityApp {
    token: String,
    sync_client: Option<SyncClient>,
    app_state: AtomicState,
    settings: Settings,
    logs: logs::Layout,
    offline_mode: bool
}

impl DiscordActivityApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
       Self::default()
    }

    fn run_sync_client(&mut self) {
        let client = SyncClient::new(self.app_state.clone(), &self.token);
        
        client.send_request( 
            to_string( &GatewayEvent::from_settings(self.settings.clone())).unwrap()
        );

        self.sync_client = Some(client);
    }

    fn disconnect(&mut self) {
        if let Some(client) = &self.sync_client {
            client.disconnect();
        }

        self.app_state.store( WebSocketState::Disconnected );
    }
}


impl eframe::App for DiscordActivityApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {    
        egui::CentralPanel::default().show(ctx, |ui| {
            let conn_state = self.app_state.load();

            self.logs.update(&conn_state);

            ui.vertical_centered(|ui| {
                ui.heading("🎮 Discord Custom Activity");
                ui.label("Configure and run your custom Discord rich presence.");
                ui.add_space(10.0);

                ui.separator();

                ui.group(|ui| {
                    ui.label("📋 Activity Settings");
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.add(egui::TextEdit::singleline(&mut self.settings.name).hint_text("Game / App Name").desired_width(200.0));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Details:");
                        ui.add(egui::TextEdit::singleline(&mut self.settings.details).hint_text("Status or detail").desired_width(200.0));
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
                        ui.add(egui::DragValue::new(&mut self.settings.r#type).clamp_range(-1..=5).speed(0.3));
                        ui.label("(0: Playing, 1: Streaming, etc.)").on_hover_text("Refer to Discord activity types");
                    });

                    ui.horizontal(|ui| {
                        ui.label("🖼 Icon:");
                        ui.label("Drag and drop an image into the app");
                    });
                });

                ui.separator();

                ui.group(|ui| {
                    ui.label("🔐 Discord Token");
                    ui.add_space(5.0);
                    ui.add_enabled_ui(!self.offline_mode, |ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.token)
                                .hint_text("Paste your token here")
                                .desired_width(300.0)
                        );
                    });
                });

                ui.add_space(7.0);
                
                
                
                ui.add_enabled( false,
                    egui::TextEdit::multiline(&mut self.logs.label)
                        .text_color( self.logs.color )
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .desired_rows(3)
                        .desired_width(f32::INFINITY)
                );

                // Mode toggle and start/stop
                ui.group(|ui| {
                    let btn_label = if conn_state == WebSocketState::Connected { "⏹ Stop" } else { "▶ Start" };
                    let button = egui::Button::new(btn_label).min_size(egui::Vec2::new(65.0, 15.0));
                    
                    ui.horizontal(|ui| {
                        ui.add_enabled( false, egui::SelectableLabel::new( self.offline_mode, "Offline_mode" ) );
                        ui.selectable_value(&mut self.offline_mode, false, "WebSocket Mode");

                        if ui.add(button).clicked() {
                            if !self.offline_mode {
                                match conn_state {
                                    WebSocketState::Connected => {
                                        self.disconnect();
                                    }
                                    WebSocketState::Disconnected => {
                                        self.run_sync_client();          
                                    }
                                    _ => {}
                                }
                            } 
                        }
                    });
                });
            });
        });
    }
}