pub mod exercise;
pub mod person;
pub mod timeslot;
pub mod utils;
pub mod work_set;

/// Contains schemas for all API requests and response
///
/// Schema always ends with Response and Request to label them
/// as api schemas, the only exception is that inner schemas
/// don't need to be named this way
pub mod schemas;

use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};
use exercise::{
    exercis_get, exercise_count_delete, exercise_count_put, exercise_delete, exercise_duplicate,
    exercise_post, exercise_put,
};
use person::get_person;
use sea_orm::DatabaseConnection;
use serde_json::{json, Value};
use timeslot::{timeslot_delete, timeslot_get, timeslot_post, timeslot_put};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use work_set::work_set_put;

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
            .route("/timeslot", get(timeslot_get))
            .route("/timeslot", post(timeslot_post))
            .route("/timeslot", delete(timeslot_delete))
            .route("/timeslot", put(timeslot_put))
            .route("/workset", put(work_set_put))
            .route("/exercise/{timeslot_id}", get(exercis_get))
            .route("/exercise", put(exercise_put))
            .route("/exercise", post(exercise_post))
            .route("/exercise", delete(exercise_delete))
            .route("/exercise/count", put(exercise_count_put))
            .route("/exercise/count", delete(exercise_count_delete))
            .route("/exercise/duplicate", post(exercise_duplicate))
            .route("/person", get(get_person))
            // TODO: Fix cors later
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http())
            .with_state(state)
    }
}

async fn liveness() -> Json<Value> {
    Json(json!({"ready": "1"}))
}
