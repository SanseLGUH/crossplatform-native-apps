use eframe::egui;

use crossbeam_channel::unbounded;

use tokio::{ 
    task::{ self, JoinHandle }, 
    sync::oneshot 
};

use smart_default::SmartDefault;

use crate::{
    client::{ self, websocket::types::AtomicState }, logs, settings::*
}; 

#[derive(SmartDefault)]
pub struct DiscordActivityApp {
    token: String,
    websocket_backend: WebsocketBackend,
    settings: Settings,
    logs: logs::Layout,
    // offline_mode: bool
}

impl DiscordActivityApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
       Self::default()
    }

    fn run_sync_client(&mut self) {
        // Ok(())
    }

    fn handle_failure(&mut self) {
        // Ok(())
    }
}


impl eframe::App for DiscordActivityApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    //     egui::CentralPanel::default().show(ctx, |ui| {
    //         let conn_state = self.websocket_backend.connection_state.load();

    //         if conn_state == ConnectionState::Failed {
    //             self.handle_failure();
    //         }

    //         ui.vertical_centered(|ui| {
    //             ui.heading("🎮 Discord Custom Activity");
    //             ui.label("Configure and run your custom Discord rich presence.");
    //             ui.add_space(10.0);

    //             ui.separator();

    //             ui.group(|ui| {
    //                 ui.label("📋 Activity Settings");
    //                 ui.add_space(5.0);

    //                 ui.horizontal(|ui| {
    //                     ui.label("Name:");
    //                     ui.add(egui::TextEdit::singleline(&mut self.settings.name).hint_text("Game / App Name").desired_width(200.0));
    //                 });

    //                 ui.horizontal(|ui| {
    //                     ui.label("Details:");
    //                     ui.add(egui::TextEdit::singleline(&mut self.settings.details).hint_text("Status or detail").desired_width(200.0));
    //                 });

    //                 ui.horizontal(|ui| {
    //                     ui.label("State:");
    //                     ui.add(egui::TextEdit::singleline(&mut self.settings.state).hint_text("Status or detail").desired_width(200.0));
    //                 });

    //                 ui.horizontal(|ui| {
    //                     ui.label("URL:");
    //                     ui.add(egui::TextEdit::singleline(&mut self.settings.url).hint_text("https://...").desired_width(200.0));
    //                 });

    //                 ui.horizontal(|ui| {
    //                     ui.label("Type:");
    //                     ui.add(egui::DragValue::new(&mut self.settings.r#type).clamp_range(-1..=5).speed(0.3));
    //                     ui.label("(0: Playing, 1: Streaming, etc.)").on_hover_text("Refer to Discord activity types");
    //                 });

    //                 ui.horizontal(|ui| {
    //                     ui.label("🖼 Icon:");
    //                     ui.label("Drag and drop an image into the app");
    //                 });
    //             });

    //             ui.separator();

    //             ui.group(|ui| {
    //                 ui.label("🔐 Discord Token");
    //                 ui.add_space(5.0);
    //                 ui.add_enabled_ui(!self.offline_mode, |ui| {
    //                     ui.add(
    //                         egui::TextEdit::singleline(&mut self.token)
    //                             .hint_text("Paste your token here")
    //                             .desired_width(300.0)
    //                     );
    //                 });
    //             });

    //             ui.add_space(7.0);
                
                
                
    //             ui.add_enabled( false,
    //                 egui::TextEdit::multiline(&mut self.logs.label)
    //                     .font(egui::TextStyle::Monospace) // for cursor height
    //                     .desired_rows(3)
    //                     .desired_width(f32::INFINITY)
    //                     .background_color( self.logs.color )
    //             );

    //             // Mode toggle and start/stop
    //             ui.group(|ui| {
    //                 let btn_label = if conn_state == ConnectionState::Connected { "⏹ Stop" } else { "▶ Start" };
    //                 let button = egui::Button::new(btn_label).min_size(egui::Vec2::new(65.0, 15.0));
                    
    //                 ui.horizontal(|ui| {
    //                     ui.add_enabled( false, egui::SelectableLabel::new( self.offline_mode, "Offline_mode" ) );
    //                     ui.selectable_value(&mut self.offline_mode, false, "WebSocket Mode");

    //                     if ui.add(button).clicked() {
    //                         if !self.offline_mode {
    //                             match conn_state {
    //                                 ConnectionState::Connected => {
    //                                     self.disconnecting_ws();
    //                                 }
    //                                 ConnectionState::Disconnected => {
    //                                     self.connecting_ws();          
    //                                 }
    //                                 _ => {}
    //                             }
    //                         } 
    //                     }
    //                 });
    //             });
    //         });
    //     });
    }
}