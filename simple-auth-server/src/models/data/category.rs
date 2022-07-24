use crate::data::common::{DataContext, DataElement, DataTools};
use serde::{Deserialize, Serialize};
use tiberius::Row;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Category {
	pub id: i32,
	pub name: String,
	pub parent_category_id: Option<i32>,
}

impl Category {
	pub fn new(id: i32, name: String, parent_category_id: Option<i32>) -> Self {
		Category {
			id,
			name,
			parent_category_id,
		}
	}

	/// Retrieves all categories from the system.
	pub async fn load_all_categories(data_context: &mut DataContext) -> Vec<Self> {
		let query = "Select * From dbo.Category;";

		Category::load_collection(&query, data_context).await
	}

	fn load_from_combined_row(id: &i32, start_index: &mut usize, row: &Row) -> Self {
		Category {
			id: *id,
			name: DataTools::get_string_and_increment(start_index, row),
			parent_category_id: DataTools::get_i32_as_option_and_increment(start_index, row),
		}
	}
}

impl DataElement for Category {
	fn populate_element_from_row(row: tiberius::Row) -> Option<Self>
	where
		Self: Sized,
	{
		let id: i32 = row.get(0).unwrap();

		Some(Category::load_from_combined_row(&id, &mut 1, &row))
	}
}
