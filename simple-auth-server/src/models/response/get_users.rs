use crate::models::data::users::User;
use serde::{Deserialize, Serialize};
use tiberius::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetUsersResponse {
	pub id: Uuid,
	pub name: String,
	pub email: String,
}

impl GetUsersResponse {
	pub fn new(id: Uuid, name: String, email: String) -> Self {
		GetUsersResponse { id, name, email }
	}

	pub fn convert_from_data_model(data_model: Vec<User>) -> Vec<Self> {
		let mut result: Vec<Self> = Vec::new();

		// Clone and update the resulting user since the password should not be returned except in special cases.
		for user in data_model.iter() {
			result.push(GetUsersResponse::new(user.id.clone(), user.name.clone(), user.email.clone()));
		}

		result
	}
}
