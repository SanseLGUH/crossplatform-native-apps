use eframe::egui;

use crossbeam_channel::unbounded;

use tokio::{ 
    task::{ self, JoinHandle }, 
    sync::oneshot 
};

use timer;
use chrono;

use crate::{
    error::{WebResult, ConnectionError}, 
    websocket, settings::*
}; 

#[derive(Default)]
pub struct DiscordActivityApp {
    token: String,
    failure_handling: bool,
    websocket_backend: WebsocketBackend,
    settings: Settings,
    offline_mode: bool
}

impl DiscordActivityApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
       Self::default()
    }

    fn connecting_ws(&mut self) -> WebResult<()> {
        let app_state = self.websocket_backend.connection_state.clone();
        let (s, r) = unbounded::<WebResult<websocket::Client>>();

        task::spawn( async move {
            app_state.store( ConnectionState::Connecting );

            let client = websocket::connect("token").await;

            match client {
                Ok(con) => {
                    
                }
                Err(e) => {
                    
                }
            }

            s.send(
                websocket::connect("token").await
            );

            app_state.store( ConnectionState::Connected )
        });

        // loop {
        //     if let Ok(resp) = r.try_recv() {
        //         println!("{:?}", resp);

        //         match resp {
        //             Ok(r) => {


        //                 return Ok(());
        //             },
        //             Err(e) => return Err( e )
        //         }


        //     }
        // }



        Ok(())
    }

    fn handle_failure(&mut self) -> WebResult<()> {
    
        Ok(())
    }

    fn disconnecting_ws(&mut self) -> WebResult<()> {
        if let Some(task) = &self.websocket_backend.task {
            task.abort();
        }
        
        if let Some(ws) = &mut self.websocket_backend.websocket {
            ws.disconnect();
        }     

        self.websocket_backend.connection_state.store( ConnectionState::Disconnected );

        Ok(())
    }
}


impl eframe::App for DiscordActivityApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let conn_state = self.websocket_backend.connection_state.load();

            if conn_state == ConnectionState::Failed {
                self.handle_failure();
            }

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
                                    ConnectionState::Connected => {
                                        self.disconnecting_ws();
                                    }
                                    ConnectionState::Disconnected => {
                                        self.connecting_ws();          
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

