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

	pub async fn load_all_users(data_context: &mut DataContext) -> Vec<Self> {
		let query = "Select * From dbo.Users;";

		User::load_collection(&query, data_context).await
	}

	pub async fn load_user_by_name_or_email(name: &str, email: &str, data_context: &mut DataContext) -> Option<Self> {
		let query = "Select * From dbo.Users WHERE Name = @P1 OR email = @P2";

		User::load_single_with_params(query, &[&name.to_owned(), &email.to_owned()], data_context).await
	}

	pub async fn insert_new(user: Self, data_context: &mut DataContext) {
		let query = "INSERT INTO dbo.Users VALUES(@P1, @P2, @P3, @P4)";

		// TODO: Add general logging for insert errors.
		match User::insert_with_params(query, &[&user.id, &user.name, &user.email, &user.password], data_context).await {
			Ok(_) => {}
			Err(error) => {
				dbg!(error);
			}
		};
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
