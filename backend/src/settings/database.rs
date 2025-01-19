use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    #[serde(default = "default_db_host")]
    pub host: String,
    #[serde(default = "default_db_password")]
    pub password: String,
    #[serde(default = "default_db_user")]
    pub user: String,
    #[serde(default = "default_db_port")]
    pub port: u16,
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
            host: default_db_host(),
            password: default_db_password(),
            port: default_db_port(),
            user: default_db_user(),
        }
    }
}

fn default_db_port() -> u16 {
    5432
}

fn default_db_user() -> String {
    "root".to_string()
}

fn default_db_host() -> String {
    "127.0.0.1".to_string()
}

fn default_db_password() -> String {
    "alpharius".to_string()
}
