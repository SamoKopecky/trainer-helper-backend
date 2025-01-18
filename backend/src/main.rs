use config::AppConfig;
use db::PostgreClient;
use std::{thread, time::Duration};

pub mod config;
pub mod db;

fn main() {
    let app_config = AppConfig::build().expect("Error building configuration");

    test_db(app_config);
}

fn test_db(app_config: AppConfig) {
    let mut pg = PostgreClient::build(&app_config).unwrap();
    for row in pg.client.query("SELECT 1", &[]).expect("cant execute") {
        let value: i32 = row.get(0);
        println!("Value is: {value}");
    }

    println!("Running ...");
    thread::sleep(Duration::MAX);
}
