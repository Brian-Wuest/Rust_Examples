use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WebInfo {
	pub host: String,
	pub port: u16,
	pub key: String,
	pub pass_secret: String,
	pub pass_salt: String,
}
