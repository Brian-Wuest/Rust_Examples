#[macro_use]
extern crate lazy_static;

use crate::data::common::DataContext;
use crate::models::config::AppConfig;
use actix_web::{App, HttpServer};
use futures::executor;

mod api;
mod data;
mod models;
mod util;

lazy_static! {
	#[derive(Debug)]
	pub static ref APP_DATA: AppConfig = load_config();

	#[derive(Debug)]
	pub static ref DATA_CONTEXT: DataContext = load_data_context();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let local_host = &APP_DATA.web_info.host;
	let local_port = &APP_DATA.web_info.port.to_string();
	let host_address = local_host.to_owned() + ":" + local_port;

	println!("{:?}", &host_address);

	HttpServer::new(|| {
		App::new()
		//.configure(CustomerController::config)
	})
	.bind(host_address)?
	.run()
	.await
}

fn load_config() -> AppConfig {
	AppConfig::new()
}

fn load_data_context() -> DataContext {
	let data_info = APP_DATA.database_info.clone();
	executor::block_on(data_info.create_pool())
}
