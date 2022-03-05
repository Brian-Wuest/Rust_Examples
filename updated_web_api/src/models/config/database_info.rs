use serde::Deserialize;
use crate::data::common::DataContext;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseInfo {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user_name: String,
    pub password: String
}

impl DatabaseInfo {
    pub async fn create_pool(self) -> DataContext {
        DataContext::new(self).await
    }
}