use std::error::Error;

use postgres::{Client, NoTls};

use crate::settings::AppConfig;

pub struct PostgreClient {
    pub client: Client,
}

impl PostgreClient {
    fn get_tls_mode(config: &AppConfig) -> NoTls {
        match config.env.as_str() {
            // TODO: Make prod require TLS
            "dev" => NoTls,
            "prod" => NoTls,
            _ => NoTls,
        }
    }

    pub fn build(config: &AppConfig) -> Result<PostgreClient, Box<dyn Error>> {
        Ok(PostgreClient {
            client: Client::connect(&config.database.conn_string(), Self::get_tls_mode(&config))?,
        })
    }
}
