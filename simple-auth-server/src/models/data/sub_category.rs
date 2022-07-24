use crate::data::common::{DataContext, DataElement, DataTools};
use serde::{Deserialize, Serialize};
use tiberius::Row;

use super::Category;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SubCategory {
	pub id: i64,
	pub name: String,
	pub parent_category_id: Option<i64>,
	pub parent_category: Option<Category>,
}

impl SubCategory {
	pub fn new(id: i64, name: String, parent_category_id: Option<i64>) -> Self {
		SubCategory {
			id,
			name,
			parent_category_id,
			parent_category: None,
		}
	}

	/// Retrieves all sub-categories from the system.
	pub async fn load_all_categories(data_context: &mut DataContext) -> Vec<Self> {
		let query = "Select * 
      From dbo.Category cat
      LEFT JOIN dbo.Category oc
        ON cat.ParentCategoryID = oc.ID";

		SubCategory::load_collection(&query, data_context).await
	}

	pub(crate) fn load_from_combined_row(id: &i64, start_index: &mut usize, row: &Row) -> Self {
		let mut parent_category: Option<Category> = None;

		if start_index < &mut (row.len() - 1) {
			let name = DataTools::get_string_and_increment(start_index, row);
			let parent_category_id = DataTools::get_i64_as_option_and_increment(start_index, row);

			// Include the details of the parent category when there are more columns to be retreived.
			if parent_category_id.is_some() && start_index < &mut (row.len() - 1) {
				*start_index += 1;
				parent_category = Some(Category::load_from_combined_row(&parent_category_id.unwrap(), start_index, row));
			}

			SubCategory {
				id: *id,
				name,
				parent_category_id,
				parent_category,
			}
		} else {
			SubCategory::default()
		}
	}
}

impl DataElement for SubCategory {
	fn populate_element_from_row(row: tiberius::Row) -> Option<Self>
	where
		Self: Sized,
	{
		let id: i64 = row.get(0).unwrap();

		Some(SubCategory::load_from_combined_row(&id, &mut 1, &row))
	}
}
