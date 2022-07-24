use serde::{Deserialize, Serialize};

use crate::models::data::Category as DataCategory;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Category {
	pub id: i64,
	pub name: String,
}

impl Category {
	pub(crate) fn convert_from_data_model(data_model: &DataCategory) -> Self {
		Category {
			id: data_model.id.clone(),
			name: data_model.name.clone(),
		}
	}
}
