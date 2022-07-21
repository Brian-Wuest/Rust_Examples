use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LoginRequest {
	pub name: String,
	pub password: String,
}
