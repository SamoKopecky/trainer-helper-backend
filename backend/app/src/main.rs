use db::Db;
use entity::prelude::*;
use entity::timeslot;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{sqlx::types::chrono::Local, EntityTrait};
use settings::AppConfig;

mod db;
mod settings;

// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/", get(|| async { "Hello, World!" }));
//
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

#[tokio::main]
async fn main() {
    let app_config = AppConfig::build().expect("Error building configuration");
    let db = Db::build(&app_config).await.unwrap();

    let naive_now = Utc::now().naive_local();
    let model = Timeslot::build(1, naive_now, 103);
    let result = Timeslot::insert(model).exec(&db.pool).await;
    println!("{:?}", result);
    println!("{:?}", timeslot::Entity::find_by_id(1).one(&db.pool).await);
}
