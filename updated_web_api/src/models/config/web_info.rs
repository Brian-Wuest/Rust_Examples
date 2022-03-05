use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WebInfo {
    pub host: String,
    pub port: u16
}