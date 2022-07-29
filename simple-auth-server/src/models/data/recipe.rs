use crate::data::common::{DataContext, DataElement, DataTools};
use serde::{Deserialize, Serialize};
use tiberius::{Row, Uuid};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Recipe {
	id: Uuid,
	user_id: Uuid,
	name: String,
	ingredients: String,
	instructions: String,
	category_id: i64,
	shared: bool,
}

impl Recipe {
	/// Retrieves all recipes from the system.
	pub async fn load_all_shared_recipes(data_context: &mut DataContext) -> Vec<Self> {
		let query = "Select * From dbo.Recipe Where Shared = 1;";

		Recipe::load_collection(&query, data_context).await
	}

	fn load_from_combined_row(id: &Uuid, start_index: &mut usize, row: &Row) -> Self {
		Recipe {
			id: *id,
			user_id: DataTools::get_uuid_and_increment(start_index, row),
			name: DataTools::get_string_and_increment(start_index, row),
			ingredients: DataTools::get_string_and_increment(start_index, row),
			instructions: DataTools::get_string_and_increment(start_index, row),
			category_id: DataTools::get_i64_and_increment(start_index, row),
			shared: DataTools::get_bool_and_increment(start_index, row),
		}
	}
}

impl DataElement for Recipe {
	fn populate_element_from_row(row: tiberius::Row) -> Option<Self>
	where
		Self: Sized,
	{
		let id: Uuid = row.get(0).unwrap();

		Some(Recipe::load_from_combined_row(&id, &mut 1, &row))
	}
}
