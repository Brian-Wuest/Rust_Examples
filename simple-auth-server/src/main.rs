#[macro_use]
extern crate lazy_static;
use crate::models::config::AppConfig;
use crate::{api::UsersController, data::common::DataContext};
use actix_identity::config::LogoutBehaviour;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use futures::executor;
use std::sync::Mutex;

// Create modules
mod api;
mod data;
mod models;
mod util;

// Create globals.
lazy_static! {
	// Basic application data.
	#[derive(Debug)]
	pub static ref APP_DATA: AppConfig = load_config();

	// Database context (pool).
	#[derive(Debug)]
	pub static ref DATA_CONTEXT: Mutex<DataContext> = load_data_context();
}

/// Gets the key from the application configuration or generates a random key if the configuration was not filled in.
/// When generating a random key; it is not persisted and users will not be able to call endpoints after an application restart without re-logging in.
fn get_key() -> Key {
	if APP_DATA.web_info.key.is_empty() {
		// Key not found in the configuration file. Generate a random key.
		let key = generate_key();

		Key::from(&key)
	} else {
		let mut key_vec: Vec<u8> = Vec::new();

		for key_value in APP_DATA.web_info.key.split(",") {
			key_vec.push(key_value.parse().unwrap());
		}

		Key::from(&key_vec)
	}
}

/// This is used to generate a random key for web cookie authorization.
fn generate_key() -> Vec<u8> {
	let key_value = Key::generate();
	let master_value = key_value.master();
	let mut key_string = "".to_string();

	// create a comma delimited string of this master key.
	for (index, value) in master_value.iter().enumerate() {
		key_string = key_string + &value.to_string();

		if index != master_value.len() - 1 {
			key_string = key_string + ","
		}
	}

	// Print this out to the console so an admin can copy/paste it into the configuration file.
	// TODO: Persist this to the configuration file automagically.
	println!("Set Environment Var: {}", &key_string);

	// Note: This is only valid for the current process. need to persist this information locally.
	std::env::set_var("sample_auth_key", key_string);

	master_value.to_vec()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// TODO: Set this to WARN or ERROR when in production modes.
	std::env::set_var("RUST_LOG", "info");
	env_logger::init();

	let local_host = &APP_DATA.web_info.host;
	let local_port = &APP_DATA.web_info.port.to_string();
	let host_address = local_host.to_owned() + ":" + local_port;

	// Get the key from the application configuration.
	let cookie_key = get_key();

	log::info!("Starting http server: {}", &host_address);

	// Create the Web Host.
	HttpServer::new(move || {
		App::new()
			.wrap(Logger::default())
			.wrap(
				IdentityMiddleware::builder()
					.logout_behaviour(LogoutBehaviour::PurgeSession)
					// Since we are using cookie sessions there isn't a way to manually expire this without custom middleware.
					//.visit_deadline(Some(std::time::Duration::from_secs(86400)))
					.build(),
			)
			.wrap(
				SessionMiddleware::builder(CookieSessionStore::default(), cookie_key.clone())
					// In production make sure this option is turned off.
					.cookie_secure(false)
					.build(),
			)
			.configure(UsersController::config)
	})
	.bind(host_address)?
	.run()
	.await
}

fn load_config() -> AppConfig {
	AppConfig::new()
}

fn load_data_context() -> Mutex<DataContext> {
	let data_info = APP_DATA.database_info.clone();
	Mutex::new(executor::block_on(data_info.create_pool()))
}
