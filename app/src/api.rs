pub mod exercise;
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
use exercise::{exercis_get, exercise_count_delete, exercise_count_put, exercise_put};
use sea_orm::DatabaseConnection;
use serde_json::{json, Value};
use timeslot::timeslot_post;
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
            .route("/timeslot", post(timeslot_post))
            .route("/workset", put(work_set_put))
            .route("/exercise/{timeslot_id}", get(exercis_get))
            .route("/exercise", put(exercise_put))
            .route("/exercise-count", put(exercise_count_put))
            .route("/exercise-count", delete(exercise_count_delete))
            // TODO: Fix cors later
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http())
            .with_state(state)
    }
}

async fn liveness() -> Json<Value> {
    Json(json!({"ready": "1"}))
}
