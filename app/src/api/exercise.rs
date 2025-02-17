use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::{to_value, Value};

use crate::crud::exercise::CRUDExercise;

use super::AppState;

pub async fn get_exercise(
    State(state): State<AppState>,
    Path(timeslot_id): Path<i32>,
) -> Json<Value> {
    let exercises = CRUDExercise::get_by_timeslot_id(&state.db, timeslot_id)
        .await
        .unwrap();

    Json(to_value(exercises).unwrap())
}
