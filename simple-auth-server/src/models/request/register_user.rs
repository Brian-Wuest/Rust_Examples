use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RegisterUserRequest {
	pub name: String,
	pub email: String,
	pub password: String,
}
