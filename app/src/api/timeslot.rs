use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::{to_value, Value};

use crate::crud::timeslot::CRUDTimeslot;

use super::{schemas::timeslot::TimeslotGetQuery, AppState};

pub async fn timeslot_get(
    State(state): State<AppState>,
    Query(params): Query<TimeslotGetQuery>,
) -> Json<Value> {
    let timeslots = CRUDTimeslot::get_by_range_date(&state.db, params.start_date, params.end_date)
        .await
        .unwrap();
    Json(to_value(timeslots).unwrap())
}
