use crate::models::config::{DatabaseInfo, WebInfo};
use config::Config;
use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "C:\\Users\\Brian\\Documents\\GitHub\\Rust_Examples\\simple-auth-server\\default.json";

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
	pub database_info: DatabaseInfo,
	pub web_info: WebInfo,
}

impl AppConfig {
	pub fn new() -> Self {
		// Load the configuration file from the file path.
		let configuration = Config::builder()
			.add_source(config::File::with_name(CONFIG_FILE_PATH))
			.build()
			.unwrap();

		match configuration.try_deserialize() {
			Ok(value) => value,
			Err(error) => {
				dbg!(error);
				panic!("See previous debug message for error");
			}
		}
	}
}
