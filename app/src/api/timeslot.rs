use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use chrono::NaiveDateTime;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::{json, to_value, Value};
use tower_http::cors::{Any, CorsLayer};

use crate::{crud::timeslot::CRUDTimeslot, db::Db};

#[derive(Deserialize, Debug)]
pub struct TimeslotsRequest {
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
}

#[derive(Clone)]
struct AppState {
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
            .route("/timeslots", post(timeslots))
            // TODO: Fix this later
            .layer(CorsLayer::permissive())
            .with_state(state)
    }
}

async fn liveness() -> Json<Value> {
    Json(json!({"ready": "1"}))
}

async fn timeslots(
    State(state): State<AppState>,
    Json(request): Json<TimeslotsRequest>,
) -> Json<Value> {
    let timeslots =
        CRUDTimeslot::get_by_range_date(&state.db, request.start_date, request.end_date)
            .await
            .unwrap();
    Json(to_value(timeslots).unwrap())
}
