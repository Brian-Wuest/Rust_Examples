use crate::models::data::users::User;
use crate::DATA_CONTEXT;
use actix_web::{
	web::{self, Json},
	HttpRequest, Result,
};
use std::borrow::BorrowMut;

pub struct UsersController {}

impl UsersController {
	pub fn config(cfg: &mut web::ServiceConfig) {
		cfg.service(web::resource("/api/users").route(web::get().to(UsersController::get_users)));
	}

	pub async fn get_users(_request: HttpRequest) -> Result<Json<Vec<User>>> {
		println!("Doing a thing");

		let mut result = Vec::new();

		match DATA_CONTEXT.lock() {
			Ok(mut context) => {
				result = User::load_all_users(context.borrow_mut()).await;
			}
			Err(err) => {
				println!("Error: {}", err);
				panic!("Error: {}", err);
			}
		}

		Ok(web::Json(result))
	}
}
