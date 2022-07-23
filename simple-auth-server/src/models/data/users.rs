use serde::Serialize;
use tiberius::{Row, Uuid};

use crate::data::common::{DataContext, DataElement, DataTools};

#[derive(Debug, Serialize, Clone, Default)]
pub struct User {
	pub id: Uuid,
	pub name: String,
	pub email: String,
	pub password: Option<Vec<u8>>,
}

impl User {
	pub fn new(name: &str, email: &str, password: &[u8]) -> Self {
		let id = Uuid::new_v4();

		User {
			id,
			name: name.to_owned(),
			email: email.to_owned(),
			password: Some(password.to_vec()),
		}
	}

	/// Loads all users registered in the system.
	pub async fn load_all_users(data_context: &mut DataContext) -> Vec<Self> {
		let query = "Select * From dbo.Users;";

		User::load_collection(&query, data_context).await
	}

	/// Loads a single user by the identifier provided.
	pub async fn load_user_by_id(id: &Uuid, data_context: &mut DataContext) -> Option<Self> {
		let query = "Select * From dbo.Users Where Id = @P1";

		User::load_single_with_params(query, &[&id.to_owned()], data_context).await
	}

	/// Loads a user where the name or email matches the supplied values.
	pub async fn load_user_by_name_or_email(name: &str, email: &str, data_context: &mut DataContext) -> Option<Self> {
		let query = "Select * From dbo.Users Where Name = @P1 OR email = @P2";

		User::load_single_with_params(query, &[&name.to_owned(), &email.to_owned()], data_context).await
	}

	/// Updates the password of an existing user.
	pub async fn update_password(id: &Uuid, new_password: Vec<u8>, data_context: &mut DataContext) -> bool {
		let query = "Update dbo.Users Set Password = @P1 Where ID = @P2";

		match User::insert_with_params(query, &[&new_password, id], data_context).await {
			Ok(_) => true,
			Err(error) => {
				log::error!("Error During User Insert: {}", error.clone());
				false
			}
		}
	}

	/// Inserts a new user into the database.
	pub async fn insert_new(user: Self, data_context: &mut DataContext) -> bool {
		let query = "Insert Into dbo.Users Values(@P1, @P2, @P3, @P4)";

		// TODO: Add general logging for insert errors.
		match User::insert_with_params(query, &[&user.id, &user.name, &user.email, &user.password], data_context).await {
			Ok(_) => true,
			Err(error) => {
				dbg!(&error);
				log::error!("Error During User Insert: {}", error.clone());
				false
			}
		}
	}

	fn load_from_combined_row(identifier: &Uuid, start_index: &mut usize, row: &Row) -> Self {
		User {
			id: *identifier,
			name: DataTools::get_string_and_increment(start_index, row),
			email: DataTools::get_string_and_increment(start_index, row),
			password: DataTools::get_varbinary_as_option_and_increment(start_index, row),
		}
	}
}

impl DataElement for User {
	fn populate_element_from_row(row: tiberius::Row) -> Option<Self>
	where
		Self: Sized,
	{
		let id: Uuid = row.get(0).unwrap();

		Some(User::load_from_combined_row(&id, &mut 1, &row))
	}
}
