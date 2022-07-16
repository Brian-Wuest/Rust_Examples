use serde::Serialize;
use tiberius::{Row, Uuid};

use crate::data::{
	common::{DataContext, DataElement, DataTools},
	date_time::SimpleDateTime,
};

#[derive(Debug, Serialize, Clone, Default)]
pub struct User {
	pub id: Uuid,
	pub email: String,
	pub hash: String,
	pub created_at: SimpleDateTime,
}

impl User {
	pub async fn load_all_users(data_context: &mut DataContext) -> Vec<Self> {
		let query = "Select * From dbo.Users;";

		User::load_collection(&query, data_context).await
	}

	fn load_from_combined_row(identifier: &Uuid, start_index: &mut usize, row: &Row) -> Self {
		User {
			id: *identifier,
			email: DataTools::get_string_and_increment(start_index, row),
			hash: DataTools::get_string_and_increment(start_index, row),
			created_at: DataTools::get_simple_date_time_and_increment(start_index, row),
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
