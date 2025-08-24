use std::fs::File;
use std::io::Write;
use std::io::Read;
use serde_json;

pub fn read_token() -> Result<String, std::io::Error> {
	let mut file = File::open("token.txt")?;
	let mut content = String::new();

	file.read_to_string(&mut content)?;

	Ok(content)
}

pub fn save_token(token: &str) -> Result<(), std::io::Error> {
	let mut file = File::create("token.txt")?;
	file.write(token.as_bytes());

	Ok(())
}