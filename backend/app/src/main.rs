use self::models::timeslot::*;
use axum::{routing::get, Router};
use crud::timeslot::TimeslotCRUD;
use db::PostgreClient;
use settings::AppConfig;
use std::{
    thread,
    time::{Duration, SystemTime},
};

pub mod crud;
mod db;
pub mod models;
pub mod schema;
mod settings;

// fn main() {
//     let app_config = AppConfig::build().expect("Error building configuration");
//     let mut pg = PostgreClient::build(&app_config).unwrap();
//
//     let new_timelots: Vec<NewTimeslot> = (1..10000)
//         .map(|i| NewTimeslot::new(i, SystemTime::now(), i * 60))
//         .collect();
//
//     let inserted_count =
//         TimeslotCRUD::insert_many(&mut pg.client, new_timelots).expect("error inserting");
//
//     println!("Inserted {} rows", inserted_count);
//
//     if let Some(timeslot) = TimeslotCRUD::get_by_id(&mut pg.client, 1000).expect("error") {
//         println!("id is : {}", timeslot.id);
//     }
//
//     println!("Running ...");
//     thread::sleep(Duration::MAX);
// }

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
