use api::timeslot::Api;
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
    let app = Api::build().await;
    let db = Db::build().await.unwrap();

    let res = Timeslot::insert_many(generate_sample_week().await)
        .exec(&db.pool)
        .await
        .unwrap();
    println!("{:?}", res);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
