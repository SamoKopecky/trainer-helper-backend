use api::timeslot::Api;
use clap::{arg, command};
use db::Db;
use entity::prelude::*;
use sea_orm::prelude::*;

use seeder::generate_sample_week;

mod api;
pub mod crud;
mod db;
mod seeder;
mod settings;

#[tokio::main]
async fn main() {
    let matches = command!()
        .arg(arg!(-s --seed "Seed db").required(false))
        .get_matches();

    match matches.get_one::<bool>("seed") {
        Some(seed) if *seed == true => {
            let db = Db::build().await.unwrap();
            let res = Timeslot::insert_many(generate_sample_week().await)
                .exec(&db.pool)
                .await
                .unwrap();
            println!("Last inserted {}", res.last_insert_id);
        }
        _ => {}
    }

    let app = Api::build().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running API ...");
    axum::serve(listener, app).await.unwrap();
}
