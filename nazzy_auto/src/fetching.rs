use reqwest::{
	blocking::get as blocking_reqwest_get, 
	get as reqwest_get,
	Error
};
use std::sync::{
	Arc, atomic::AtomicU32
};
use serde::{
	Serialize, Deserialize
};
use crossbeam::atomic::AtomicCell;

use crate::{ 
	structs::{MiniGameCollection, METADATA}
};


pub fn metadata() -> Result<MiniGameCollection, Error> {
	Ok( 
		blocking_reqwest_get(METADATA)?.json()? 
	)
}

// error handeling is in state
pub fn install(progress: Arc<AtomicU32>) {	

}