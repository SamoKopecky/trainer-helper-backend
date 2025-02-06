use api::timeslot::Api;
use db::Db;
use entity::timeslot;
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::chrono::Utc;

use chrono::TimeDelta;
use entity::prelude::*;

mod api;
pub mod crud;
mod db;
mod settings;

#[tokio::main]
async fn main() {
    let app = Api::build().await;
    let models = generate_sample_week().await;
    let db = Db::build().await.unwrap();
    let res = Timeslot::insert_many(models).exec(&db.pool).await;
    println!("{:?}", res);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn generate_sample_week() -> Vec<timeslot::ActiveModel> {
    let mut models: Vec<timeslot::ActiveModel> = vec![];
    let time = Utc::now().naive_local();
    const TRAINER_ID: i32 = 1;
    const DURATION: i32 = 60;

    for _ in 0..7 {
        let time = time + TimeDelta::days(1);
        models.push(Timeslot::build(TRAINER_ID, time, DURATION));
    }
    println!("{:?}", models);
    models
}
