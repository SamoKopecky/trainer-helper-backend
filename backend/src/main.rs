use db::PostgreClient;
use diesel::dsl::sql;
use diesel::pg::Pg;
use diesel::sql_types::Integer;
use diesel::{PgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use settings::AppConfig;
use std::error::Error;
use std::i32;
use std::{thread, time::Duration};

pub mod db;
mod schema;
pub mod settings;
// const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

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

    // let _ = run_migrations(&mut pg.client);

    println!("Running ...");
    thread::sleep(Duration::MAX);
}

// fn run_migrations(connection: &PgConnection) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//     connection.run_pending_migrations(MIGRATIONS)?;
//
//     Ok(())
// }
