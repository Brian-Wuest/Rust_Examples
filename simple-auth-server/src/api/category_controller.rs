use crate::models::data::SubCategory;
use crate::models::response::GetCategoryResponse;
use crate::DATA_CONTEXT;
use actix_identity::Identity;
use actix_web::{
	web::{self, Json},
	HttpRequest, Result,
};

pub struct CategoryController {}

impl CategoryController {
	pub fn config(cfg: &mut web::ServiceConfig) {
		// It's not obvious in the current implementation but you can specify multiple HTTP methods for a specific resource.
		// You can specify multiple ".route" calls for different HTTP methods to point to different handlers!
		cfg.service(web::resource("/api/category").route(web::get().to(CategoryController::index)));
	}

	async fn index(_user: Identity, _req: HttpRequest) -> Result<Json<Vec<GetCategoryResponse>>> {
		let mut result = Vec::new();
		log::info!("Loading all categories");

		match DATA_CONTEXT.lock() {
			Ok(mut context) => {
				let data_result = SubCategory::load_all_categories(&mut context).await;

				result = GetCategoryResponse::convert_from_data_model(data_result);
			}
			Err(err) => {
				println!("Error: {}", err);
				log::error!("Error:, {}", err);
				panic!("Error: {}", err);
			}
		}

		Ok(web::Json(result))
	}
}
