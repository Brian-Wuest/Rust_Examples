use std::str::FromStr;

use crate::models::config::DatabaseInfo;
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use tiberius::Config;

#[derive(Debug, Clone)]
pub struct DataContext {
	pub connection_pool: Pool<ConnectionManager>,
}

impl DataContext {
	pub async fn new(data_info: DatabaseInfo) -> Self {
		let mut connection_string: String = String::from_str("").unwrap();
		let mut using_named_connection = false;

		if data_info.port == 0 {
			if data_info.instance.is_empty() {
				connection_string = format!("Server=tcp:{};Initial Catalog={};Persist Security Info=False;User ID={};Password={};MultipleActiveResultSets=False;Encrypt={};TrustServerCertificate={};Connection Timeout={};",
              &data_info.host,
              &data_info.database,
              &data_info.user_name,
              &data_info.password,
              &data_info.encrypt_connection,
              &data_info.trust_server_cert,
              &data_info.connection_timeout);
			} else {
				connection_string = format!("Server=tcp:{}\\{};Initial Catalog={};Persist Security Info=False;User ID={};Password={};MultipleActiveResultSets=False;Encrypt={};TrustServerCertificate={};Connection Timeout={};",
              &data_info.host,
              &data_info.instance,
              &data_info.database,
              &data_info.user_name,
              &data_info.password,
              &data_info.encrypt_connection,
              &data_info.trust_server_cert,
              &data_info.connection_timeout);

				using_named_connection = true;
			}
		} else {
			connection_string = format!("Server=tcp:{},{};Initial Catalog={};Persist Security Info=False;User ID={};Password={};MultipleActiveResultSets=False;Encrypt={};TrustServerCertificate={};Connection Timeout={};",
          &data_info.host,
          &data_info.port,
          &data_info.database,
          &data_info.user_name,
          &data_info.password,
          &data_info.encrypt_connection,
          &data_info.trust_server_cert,
          &data_info.connection_timeout);
		}

		DataContext::new_from_connection(connection_string, using_named_connection).await
	}

	pub async fn new_from_connection(connection_string: String, using_named_connection: bool) -> Self {
		let config = Config::from_ado_string(&connection_string).unwrap();

		let mut connection_manager = bb8_tiberius::ConnectionManager::new(config.clone()).using_named_connection();

		if using_named_connection {
			connection_manager = connection_manager.using_named_connection();
		}

		let connection_pool = bb8::Pool::builder().max_size(2).build(connection_manager).await.unwrap();
		match connection_pool.dedicated_connection().await {
			Ok(_) => {}
			Err(error) => {
				dbg!("There was an error connecting to the database");
				dbg!(error);
				panic!("See above information for error details");
			}
		};

		DataContext {
			connection_pool: connection_pool,
		}
	}
}
