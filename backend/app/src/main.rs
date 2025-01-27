use entity::prelude::*;
use entity::timeslot;
use sea_orm::{sqlx::types::chrono::Local, ActiveValue::NotSet, EntityTrait, Set};
use settings::AppConfig;

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
    let connection = sea_orm::Database::connect(app_config.database.conn_string())
        .await
        .unwrap();

    let chrono_now = Local::now().naive_local();
    let model: timeslot::ActiveModel = timeslot::ActiveModel {
        id: NotSet,
        trainer_id: Set(1),
        start: Set(chrono_now),
        duration: Set(601),
        updated_at: Set(chrono_now),
        created_at: Set(chrono_now),
        user_id: Set(Some(1)),
    }
    .into();

    let timeslot = Timeslot::insert(model).exec(&connection).await;
    println!("{:?}", timeslot);
    println!(
        "{:?}",
        timeslot::Entity::find_by_id(1).one(&connection).await
    );
}
