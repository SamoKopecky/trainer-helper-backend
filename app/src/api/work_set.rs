use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::{to_value, Value};

use crate::crud::work_set::CRUDWorkSet;

use super::AppState;

#[derive(Deserialize, Debug)]
pub struct WorkSetRequest {
    timeslot_id: i32,
}

pub async fn work_set_api(
    State(state): State<AppState>,
    Json(request): Json<WorkSetRequest>,
) -> Json<Value> {
    let work_sets = CRUDWorkSet::get_by_timeslot_id(&state.db, request.timeslot_id)
        .await
        .unwrap();
    Json(to_value(work_sets).unwrap())
}
