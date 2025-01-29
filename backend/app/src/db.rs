use std::error::Error;

use sea_orm::{Database, DatabaseConnection};

use crate::settings::AppConfig;

pub struct Db {
    pub pool: DatabaseConnection,
}

impl Db {
    pub async fn build(config: &AppConfig) -> Result<Db, Box<dyn Error>> {
        Ok(Db {
            pool: Database::connect(config.database.conn_string()).await?,
        })
    }
}
