pub mod timeslot;
pub mod work_set;

use axum::{
    routing::{get, post},
    Json, Router,
};
use sea_orm::DatabaseConnection;
use serde_json::{json, Value};
use timeslot::timeslots_api;
use tower_http::cors::CorsLayer;
use work_set::work_set_api;

use crate::db::Db;

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

pub struct Api;

impl Api {
    pub async fn build() -> Router {
        let state = AppState {
            db: Db::build().await.unwrap().pool,
        };

        Router::new()
            .route("/liveness", get(liveness))
            .route("/timeslots", post(timeslots_api))
            .route("/worksets", post(work_set_api))
            // TODO: Fix this later
            .layer(CorsLayer::permissive())
            .with_state(state)
    }
}

async fn liveness() -> Json<Value> {
    Json(json!({"ready": "1"}))
}
