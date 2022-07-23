use crate::models::request::users::ChangePasswordRequest;
use crate::models::request::users::LoginRequest;
use crate::models::response::GetUsersResponse;
use crate::models::{data::users::User, request::users::RegisterUserRequest};
use crate::util::auth_services;
use crate::DATA_CONTEXT;
use actix_identity::Identity;
use actix_web::error::ErrorBadRequest;
use actix_web::{
	web::{self, Json},
	HttpMessage, HttpRequest, HttpResponse, Responder, Result,
};
use std::borrow::BorrowMut;
use tiberius::Uuid;

pub struct UsersController {}

impl UsersController {
	pub fn config(cfg: &mut web::ServiceConfig) {
		// It's not obvious in the current implementation but you can specify multiple HTTP methods for a specific resource.
		// You can specify multiple ".route" calls for different HTTP methods to point to different handlers!
		cfg.service(web::resource("/api").route(web::get().to(UsersController::index)));
		cfg.service(web::resource("/api/users").route(web::get().to(UsersController::get_users)));
		cfg.service(web::resource("/api/users/_register").route(web::post().to(UsersController::register)));
		cfg.service(web::resource("/api/users/_login").route(web::post().to(UsersController::login)));
		cfg.service(web::resource("/api/users/_change_pwd").route(web::put().to(UsersController::change_password)));
		cfg.service(web::resource("/api/users/_logout").route(web::post().to(UsersController::logout)));
	}

	// By using the "Option<Identity>" property we can have special functionality based
	// On whether or not the user is logged in or not.
	async fn index(user: Option<Identity>, _req: HttpRequest) -> Result<String> {
		if let Some(user) = user {
			// This is an example of how to get some meta data out of the current session cookie
			//let user_code: String = session.get("code").unwrap().unwrap();

			let welcome_msg = format!("Welcome! {}", user.id().unwrap());
			Ok(welcome_msg)
		} else {
			Ok("Welcome Anonymous!".to_owned())
		}
	}

	async fn register(form: Json<RegisterUserRequest>, request: HttpRequest) -> Result<String> {
		// TODO: Try to figure out how to limit the number of registrations can happen from the same IP.
		let mut context = DATA_CONTEXT.lock().unwrap();
		let name = form.name.clone();
		let email = form.email.clone();
		let form_pass = form.password.clone();

		match User::load_user_by_name_or_email(&name, &email, &mut context).await {
			Some(_) => Err(ErrorBadRequest("User registration already exists")),
			None => {
				// user does not exist, able to create user entry.
				// Create the password hash.
				match auth_services::hash_password(form_pass) {
					Ok(pass) => {
						let user = User::new(&name, &email, pass.as_bytes());

						// Insert the user information.
						if User::insert_new(user, &mut context).await {
							// This needs to be done because the insert statement doesn't provide the id after inserting the record.
							match User::load_user_by_name_or_email(&name, &email, &mut context).await {
								Some(user) => {
									// Log the user in so they get the session cookie for future requests.
									Identity::login(&request.extensions(), user.id.to_string()).unwrap();
								}
								None => {}
							};
						}
					}
					Err(error) => {
						return Err(ErrorBadRequest(error));
					}
				}

				Ok("User Registered Successfully!".to_owned())
			}
		}
	}

	async fn login(request: HttpRequest, form: Json<LoginRequest>) -> impl Responder {
		let mut context = DATA_CONTEXT.lock().unwrap();
		match User::load_user_by_name_or_email(&form.name, &form.name, &mut context).await {
			Some(user) => {
				// This is a valid user, now check the password.
				match user.password {
					Some(pass) => {
						let hash = String::from_utf8(pass).unwrap();

						match auth_services::verify(&hash, &form.password) {
							Ok(verification) => {
								// Don't do anything on the else because it will fall through to the login below.
								if !verification {
									return HttpResponse::BadRequest();
								}
							}
							Err(_error) => {
								return HttpResponse::BadRequest();
							}
						}
					}
					None => {
						// The user doesn't have a password (for some reason), so this is fine.
					}
				};

				// Log the user in so they get the session cookie for future requests.
				// attach a verified user identity to the active session
				Identity::login(&request.extensions(), user.id.to_string()).unwrap();

				HttpResponse::Ok()
			}
			None => HttpResponse::BadRequest(),
		}

		// Example of how to add some meta data to the session.
		// the session object is a parameter of the function.
		/* match session.insert("code", "ara01") {
			Ok(_) => HttpResponse::Ok(),
			Err(_error) => HttpResponse::BadRequest(),
		} */
	}

	// Need to have a valid identity in order to change the password.
	async fn change_password(user: Identity, form: Json<ChangePasswordRequest>) -> impl Responder {
		let user_id = Uuid::parse_str(&user.id().unwrap()).unwrap();
		let mut context = DATA_CONTEXT.lock().unwrap();
		let old_password = form.old_password.clone();
		let new_password = form.new_password.clone();

		match User::load_user_by_id(&user_id, &mut context).await {
			Some(user) => {
				// validate that the current user has a password because it is currently not required in the system.
				match user.password {
					Some(pass) => {
						let old_hashed_password = String::from_utf8(pass).unwrap();

						// Verify the provided "old_password" against the current user password.
						match auth_services::verify(&old_hashed_password, &old_password) {
							Ok(verification) => {
								if !verification {
									// Passwords don't match return as a bad request.
									return HttpResponse::BadRequest();
								} else {
									// Old password matches the current password. Okay to change the password.
									// Make sure that the hashing of the new password works.
									match auth_services::hash_password(new_password) {
										Ok(new_hashed_password) => {
											// Make sure the new hashed password and the old hashed password aren't the same.
											if new_hashed_password == old_hashed_password {
												return HttpResponse::BadRequest();
											}

											// Attempt to update the database with the new password.
											if !User::update_password(&user_id, new_hashed_password.into_bytes(), &mut context).await {
												return HttpResponse::BadRequest();
											}
										}
										Err(_) => {
											return HttpResponse::BadRequest();
										}
									};
								}
							}
							Err(_error) => {
								return HttpResponse::BadRequest();
							}
						};
					}
					None => {
						// The user doesn't have a password to begin with so just return okay, nothing changed.
					}
				}

				HttpResponse::Ok()
			}
			None => HttpResponse::BadRequest(),
		}
	}

	// In order to log out the user needs to be logged in (having the cookie).
	async fn logout(user: Identity) -> Result<String> {
		let user_id = user.id().unwrap().clone();
		user.logout();

		let logout_msg = format!("User With ID: {} logged out successfully!", user_id);

		Ok(logout_msg)
	}

	// To make protected routes, add the "user: Identity" parameter to the function.
	// This will force the route to have an authenticated user.
	// If the Identity is "None", the client will get a 401 (Unauthorized) response.
	async fn get_users(_user: Identity, _request: HttpRequest) -> Result<Json<Vec<GetUsersResponse>>> {
		println!("Doing a thing");

		let mut result = Vec::new();

		match DATA_CONTEXT.lock() {
			Ok(mut context) => {
				let data_result = User::load_all_users(context.borrow_mut()).await;

				result = GetUsersResponse::convert_from_data_model(data_result);
			}
			Err(err) => {
				println!("Error: {}", err);
				panic!("Error: {}", err);
			}
		}

		Ok(web::Json(result))
	}
}
