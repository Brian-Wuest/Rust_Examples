use crate::models::config::{DatabaseInfo, WebInfo};
use config::Config;
use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "C:\\Users\\Brian\\Documents\\GitHub\\Rust_Examples\\updated_web_api\\src\\default.json";

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig{
    pub database_info: DatabaseInfo,
    pub web_info: WebInfo
}

impl AppConfig {
    pub fn new() -> Self {
        // Load the configuration file from the file path.
        let configuration = Config::builder()
            .add_source(config::File::with_name(CONFIG_FILE_PATH))
            .build()
            .unwrap();

        configuration.try_deserialize().unwrap()
    }
}