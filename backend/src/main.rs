use db::PostgreClient;
use diesel::dsl::sql;
use diesel::sql_types::Integer;
use diesel::RunQueryDsl;
use settings::AppConfig;
use std::i32;
use std::{thread, time::Duration};

pub mod db;
mod schema;
pub mod settings;

fn main() {
    let app_config = AppConfig::build().expect("Error building configuration");

    test_db(app_config);
}

fn test_db(app_config: AppConfig) {
    let mut pg = PostgreClient::build(&app_config).unwrap();
    let res: Vec<i32> = sql::<Integer>("SELECT 1;")
        .load(&mut pg.client)
        .expect("failed");

    for row in res {
        println!("Value is: {row}");
    }

    println!("Running ...");
    thread::sleep(Duration::MAX);
}
