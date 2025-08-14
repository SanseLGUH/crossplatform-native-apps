use smart_default::SmartDefault;
use eframe::egui::Color32;
// use crate::{error::ConnectionError, settings::ConnectionState};

use std::fmt::Write;

#[derive(SmartDefault)]
pub struct Layout {
	#[default = "Displays the current program status. 
Please enter your Discord token."]
	pub label: String,

	#[default(_code = "Color32::from_gray(10)")]
	pub color: Color32,

	// pub connection_state: Arc<AtomicCell<ConnectionState>>
}

// disconnected is default
impl Layout {
	pub fn connected(&mut self) {
		write!( &mut self.label, "Connected.. (State)" );
	}

	pub fn connecting(&mut self) {
		// use WRITE insted
		self.label = String::from( "Connecting! Wait" );
	}

	// pub fn failed(&mut self, e: ConnectionError) {
	// 	self.label = e.to_string();
	// 	// self.color = Color32::RED;
	// }
}