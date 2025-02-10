use std::error::Error;

use sea_orm::{Database, DatabaseConnection};

use crate::settings::AppConfig;

pub struct Db {
    pub pool: DatabaseConnection,
}

impl Db {
    pub async fn build() -> Result<Db, Box<dyn Error>> {
        let app_config = AppConfig::build().expect("Error building configuration");
        Ok(Db {
            pool: Database::connect(app_config.database.conn_string()).await?,
        })
    }
}
