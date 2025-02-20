pub mod exercise;
pub mod timeslot;
pub mod utils;
pub mod work_set;

use axum::{
    routing::{get, post, put},
    Json, Router,
};
use exercise::{exercise_update, get_exercise};
use sea_orm::DatabaseConnection;
use serde_json::{json, Value};
use timeslot::timeslots_api;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use work_set::work_set_update;

use crate::db::Db;

// TODO: Make endpoints retrun a proper type not just json
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
            .route("/worksets", put(work_set_update))
            .route("/exercises/{timeslot_id}", get(get_exercise))
            .route("/exercises", put(exercise_update))
            // TODO: Fix this later
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http())
            .with_state(state)
    }
}

async fn liveness() -> Json<Value> {
    Json(json!({"ready": "1"}))
}
