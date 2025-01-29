use api::timeslot::Api;
use crud::timeslot::CRUDTimeslot;
use db::Db;
use entity::timeslot;
use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::chrono::Utc;
use settings::AppConfig;

use chrono::prelude::*;
use entity::prelude::*;

mod api;
pub mod crud;
mod db;
mod settings;

#[tokio::main]
async fn main() {
    let app = Api::build();
    // test_db().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn test_db() {
    let app_config = AppConfig::build().expect("Error building configuration");
    let db = Db::build(&app_config).await.unwrap();

    let naive_now = Utc::now().naive_local();
    let model = Timeslot::build(1, naive_now, 103);
    let result = Timeslot::insert(model).exec(&db.pool).await;
    println!("{:?}", result);
    println!("{:?}", timeslot::Entity::find_by_id(1).one(&db.pool).await);

    let start_date_time = Utc
        .with_ymd_and_hms(2025, 1, 27, 00, 00, 00)
        .unwrap()
        .naive_local();
    let end_date_time = Utc
        .with_ymd_and_hms(2025, 1, 27, 23, 00, 00)
        .unwrap()
        .naive_local();
    let result = CRUDTimeslot::get_by_range_date(&db.pool, start_date_time, end_date_time)
        .await
        .unwrap();
    println!("{}", result.len());
    for r in result {
        println!("{:?}\n\n", r);
    }
}
