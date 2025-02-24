use axum::{extract::State, Json};
use serde_json::{to_value, Value};

use crate::crud::timeslot::CRUDTimeslot;

use super::{schemas::timeslot::TimeslotPutRequest, AppState};

pub async fn timeslot_post(
    State(state): State<AppState>,
    Json(request): Json<TimeslotPutRequest>,
) -> Json<Value> {
    let timeslots =
        CRUDTimeslot::get_by_range_date(&state.db, request.start_date, request.end_date)
            .await
            .unwrap();
    Json(to_value(timeslots).unwrap())
}
