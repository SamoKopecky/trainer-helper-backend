use axum::{extract::State, Json};
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde_json::{to_value, Value};

use crate::crud::timeslot::CRUDTimeslot;

use super::AppState;

#[derive(Deserialize, Debug)]
pub struct TimeslotsRequest {
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
}

pub async fn timeslots_api(
    State(state): State<AppState>,
    Json(request): Json<TimeslotsRequest>,
) -> Json<Value> {
    let timeslots =
        CRUDTimeslot::get_by_range_date(&state.db, request.start_date, request.end_date)
            .await
            .unwrap();
    Json(to_value(timeslots).unwrap())
}
