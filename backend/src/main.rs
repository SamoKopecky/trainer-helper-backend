use config::AppConfig;
use db::get_conn;
use std::{thread, time::Duration};

pub mod config;
pub mod db;

fn main() {
    let app_config = AppConfig::build().expect("Error building configuration");

    test_db(app_config);
}

fn test_db(app_config: AppConfig) {
    thread::sleep(Duration::new(2, 0));
    let mut conn = get_conn(app_config).expect("Can't create conn");
    for row in conn.query("SELECT 1", &[]).expect("cant execute") {
        let value: i32 = row.get(0);
        println!("Value is: {value}");
    }

    println!("Running ...");
    thread::sleep(Duration::MAX);
}
