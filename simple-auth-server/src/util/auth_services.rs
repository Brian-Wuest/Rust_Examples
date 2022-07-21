use argon2::{self, Config};

use crate::APP_DATA;

pub fn hash_password(password: String) -> Result<String, &'static str> {
	let secret = APP_DATA.web_info.pass_secret.clone();
	let salt = APP_DATA.web_info.pass_salt.clone();

	let config = Config {
		secret: secret.as_bytes(),
		..Default::default()
	};

	argon2::hash_encoded(password.as_bytes(), &salt.as_bytes(), &config).map_err(|err| {
		dbg!(err);
		"Internal Server Error"
	})
}

pub fn verify<'a>(hash: &'a str, password: &'a str) -> Result<bool, &'a str> {
	let secret = APP_DATA.web_info.pass_secret.clone();

	argon2::verify_encoded_ext(hash, password.as_bytes(), secret.as_bytes(), &[]).map_err(|err| {
		dbg!(err);
		"Unauthorized"
	})
}
