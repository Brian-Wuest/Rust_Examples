use crate::{models::data::Recipe, DATA_CONTEXT};
use actix_identity::Identity;
use actix_web::{
	web::{self, Json},
	HttpRequest, Result,
};

pub struct RecipeController {}

impl RecipeController {
	pub fn config(cfg: &mut web::ServiceConfig) {
		// It's not obvious in the current implementation but you can specify multiple HTTP methods for a specific resource.
		// You can specify multiple ".route" calls for different HTTP methods to point to different handlers!
		cfg.service(web::resource("/api/recipe").route(web::get().to(RecipeController::index)));
	}

	async fn index(_user: Identity, _req: HttpRequest) -> Result<Json<Vec<Recipe>>> {
		let mut result = Vec::new();
		log::info!("Loading all recipes");

		match DATA_CONTEXT.lock() {
			Ok(mut context) => {
				result = Recipe::load_all_shared_recipes(&mut context).await;
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
