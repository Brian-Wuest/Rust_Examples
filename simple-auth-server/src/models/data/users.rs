use tiberius::{Row, Uuid};

use crate::data::{
	common::{DataContext, DataElement, DataTools},
	date_time::SimpleDateTime,
};

#[derive(Debug, Clone, Default)]
pub struct Users {
	pub id: Uuid,
	pub email: String,
	pub hash: String,
	pub created_at: SimpleDateTime,
}

impl Users {
	pub async fn load_all_users(data_context: &mut DataContext) -> Vec<Self> {
		let query = "Select * From dbo.Users;";

		Users::load_collection(&query, data_context).await
	}

	fn load_from_combined_row(identifier: &Uuid, start_index: &mut usize, row: &Row) -> Self {
		Users {
			id: *identifier,
			email: DataTools::get_string_and_increment(start_index, row),
			hash: DataTools::get_string_and_increment(start_index, row),
			created_at: DataTools::get_simple_date_time_and_increment(start_index, row),
		}
	}
}

impl DataElement for Users {
	fn populate_element_from_row(row: tiberius::Row) -> Option<Self>
	where
		Self: Sized,
	{
		let id: Uuid = row.get(0).unwrap();

		Some(Users::load_from_combined_row(&id, &mut 1, &row))
	}
}
