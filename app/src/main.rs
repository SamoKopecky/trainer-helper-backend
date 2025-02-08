use api::Api;
use clap::{arg, command};
use db::Db;
use entity::prelude::*;
use sea_orm::prelude::*;

use seeder::{generate_sample_week, generate_work_sets_in_timeslots};

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
        Some(seed) if *seed == true => insert_seeds().await,
        _ => {}
    }

    let app = Api::build().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running API ...");
    axum::serve(listener, app).await.unwrap();
}

async fn insert_seeds() {
    let db = Db::build().await.unwrap();
    let timeslots = generate_sample_week();
    let res = Timeslot::insert_many(timeslots)
        .exec(&db.pool)
        .await
        .unwrap();
    println!("Last inserted timeslot {}", res.last_insert_id);

    let sets = generate_work_sets_in_timeslots(res.last_insert_id);
    let res = WorkSet::insert_many(sets).exec(&db.pool).await.unwrap();
    println!("Last inserted set {}", res.last_insert_id);
}
