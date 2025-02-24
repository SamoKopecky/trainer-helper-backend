use config::{Config, ConfigError, Environment};
use database::Database;
use serde_derive::Deserialize;

mod database;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub database: Database,
    // #[serde(default = "default_env")]
    // pub env: String,
}

impl AppConfig {
    pub fn build() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?
            .try_deserialize()
    }
}

// fn default_env() -> String {
//     "dev".to_string()
// }
