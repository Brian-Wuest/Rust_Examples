use bb8::{Pool};
use bb8_tiberius::ConnectionManager;
use crate::models::config::DatabaseInfo;
use tiberius::{Config, AuthMethod};

#[derive(Debug, Clone)]
pub struct DataContext{
    pub connection_pool: Pool<ConnectionManager>
}

impl DataContext {
    pub async fn new(data_info: DatabaseInfo) -> Self {
        dbg!(&data_info);

        let mut config = Config::new();
        config.host(data_info.host);
        config.port(data_info.port);
        config.database(data_info.database);
        config.authentication(AuthMethod::sql_server(data_info.user_name, data_info.password));
        config.application_name("Cool Rust App");

        // This is needed if there is no certificate on the local instance. Don't use this in production.
        config.trust_cert();

        println!("Creating connection manager for SQL connection");

        let connection_manager = bb8_tiberius::ConnectionManager::new(config).using_named_connection();

        println!("Creating connection pool");

        let connection_pool = bb8::Pool::builder().max_size(100).build(connection_manager).await;

        match connection_pool {
            Err(message) => {
                dbg!(message);

                panic!("Connection errored out");
            }
            Ok(resulting_pool) => {
                println!("Created connection pool");

                return DataContext {
                    connection_pool: resulting_pool
                };
            }
        }
    }
}