use std::error::Error;

use diesel::{Connection, PgConnection};

use crate::settings::AppConfig;

pub struct PostgreClient {
    pub client: PgConnection,
}

impl PostgreClient {
    pub fn build(config: &AppConfig) -> Result<PostgreClient, Box<dyn Error>> {
        let db_url = &config.database.conn_string();

        Ok(PostgreClient {
            client: PgConnection::establish(db_url)?,
        })
    }
}
