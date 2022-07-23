use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ChangePasswordRequest {
	#[serde(rename = "oldPassword")]
	pub old_password: String,

	#[serde(rename = "newPassword")]
	pub new_password: String,
}
