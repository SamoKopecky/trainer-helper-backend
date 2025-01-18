use std::error::Error;

use postgres::{Client, NoTls};

use crate::config::AppConfig;

pub fn get_conn(config: AppConfig) -> Result<Client, Box<dyn Error>> {
    Ok(Client::connect(&config.database.conn_string(), NoTls)?)
}
