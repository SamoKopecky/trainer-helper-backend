use config::{Config, ConfigError, Environment};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub port: u16,
    pub host: String,
    pub password: String,
    pub user: String,
}

impl Database {
    pub fn conn_string(&self) -> String {
        format!(
            "host={} user={} password={}",
            self.host, self.user, self.password
        )
    }
}

impl Default for Database {
    fn default() -> Self {
        Database {
            port: 5432,
            host: "127.0.0.1".to_string(),
            user: "root".to_string(),
            password: "alpharius".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_env")]
    pub env: String,
    #[serde(default)]
    pub database: Database,
}

impl AppConfig {
    pub fn build() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(Environment::with_prefix("GOACH"))
            .build()?
            .try_deserialize()
    }
}

fn default_env() -> String {
    "dev".to_string()
}
