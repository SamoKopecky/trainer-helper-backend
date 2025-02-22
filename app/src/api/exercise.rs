use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use entity::exercise::{self};
use serde_json::{to_value, Value};

use crate::crud::exercise::CRUDExercise;

use super::{
    schemas::exercise::{ExerciseGetResponse, ExercisePutRequest, ExerciseWorkSet},
    utils::{active, handle_crud_result},
    AppState,
};

pub async fn exercis_get(
    State(state): State<AppState>,
    Path(timeslot_id): Path<i32>,
) -> Json<Value> {
    let mut res: HashMap<i32, ExerciseGetResponse> = HashMap::new();
    let exercises = CRUDExercise::get_by_timeslot_id(&state.db, timeslot_id)
        .await
        .unwrap();
    exercises.iter().for_each(|exercise| {
        res.entry(exercise.exercise_id)
            .and_modify(|response| {
                response
                    .work_sets
                    .push(ExerciseWorkSet::from_crud_model(exercise));
                response.work_set_count += 1
            })
            .or_insert(ExerciseGetResponse::from_crud_model(exercise));
    });

    let mut res_values: Vec<ExerciseGetResponse> = res.into_values().collect();
    res_values.sort_by_key(|k| (k.group_id, k.exercise_id));
    Json(to_value(res_values).unwrap())
}

pub async fn exercise_put(
    State(state): State<AppState>,
    Json(request): Json<ExercisePutRequest>,
) -> StatusCode {
    let update_model = exercise::ActiveModel {
        note: active(request.note.map(Some)),
        ..Default::default()
    };

    handle_crud_result(CRUDExercise::update_by_id(&state.db, request.id, update_model).await)
}
