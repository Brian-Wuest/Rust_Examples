use crate::models::data::users::User;
use crate::DATA_CONTEXT;
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{
	web::{self, Json},
	HttpMessage, HttpRequest, HttpResponse, Responder, Result,
};
use std::borrow::BorrowMut;

pub struct UsersController {}

impl UsersController {
	pub fn config(cfg: &mut web::ServiceConfig) {
		// It's not obvious in the current implementation but you can specify multiple HTTP methods for a specific path.
		// You can specify multiple ".route" calls for different HTTP methods to point to different handlers!
		cfg.service(web::resource("/api/users").route(web::get().to(UsersController::get_users)));
		cfg.service(web::resource("/api").route(web::get().to(UsersController::index)));
		cfg.service(web::resource("/api/login").route(web::get().to(UsersController::login)));
		cfg.service(web::resource("/api/logout").route(web::get().to(UsersController::logout)));
	}

	async fn login(session: Session, request: HttpRequest) -> impl Responder {
		// Some kind of authentication should happen here
		// e.g. password-based, biometric, etc.
		// [...]

		// attach a verified user identity to the active session
		Identity::login(&request.extensions(), "User1".into()).unwrap();
		session.insert("code", "ara01");

		HttpResponse::Ok()
	}

	// By using the "Option<Identity>" property we can have special functionality based
	// On whether or not the user is logged in or not.
	async fn index(user: Option<Identity>, session: Session, req: HttpRequest) -> Result<String> {
		log::info!("{req:?}");

		if let Some(user) = user {
			let user_code: String = session.get("code").unwrap().unwrap();

			let welcome_msg = format!("Welcome! {}, Code: {}", user.id().unwrap(), user_code);
			Ok(welcome_msg)
		} else {
			Ok("Welcome Anonymous!".to_owned())
		}
	}

	// In order to log out the user needs to be logged in (having the cookie).
	async fn logout(user: Identity, session: Session) -> Result<String> {
		let user_code: String = session.get("code").unwrap().unwrap();
		user.logout();

		let logout_msg = format!("User With Code: {} logged out successfully!", user_code);

		Ok(logout_msg)
	}

	// To make protected routes, add the "user: Identity" parameter to the function.
	// This will force the route to have an authenticated user.
	// If the Identity is "None", the client will get a 401 (Unauthorized) response.
	async fn get_users(_user: Identity, _request: HttpRequest) -> Result<Json<Vec<User>>> {
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
