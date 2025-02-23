use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use entity::{
    exercise::{self},
    work_set,
};
use sea_orm::{ActiveValue::NotSet, IntoActiveModel};

use crate::crud::{exercise::CRUDExercise, work_set::CRUDWorkSet};

use super::{
    schemas::exercise::{
        ExerciseCountDeleteRequest, ExerciseCountPutRequest, ExerciseGetResponse,
        ExercisePutRequest, ExerciseWorkSet,
    },
    utils::{active, handle_crud_result},
    AppState,
};

pub async fn exercis_get(
    State(state): State<AppState>,
    Path(timeslot_id): Path<i32>,
) -> Json<Vec<ExerciseGetResponse>> {
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
    Json(res_values)
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

pub async fn exercise_count_put(
    State(state): State<AppState>,
    Json(request): Json<ExerciseCountPutRequest>,
) -> Json<Vec<work_set::Model>> {
    let copy_model = request.work_set_template.to_active_model(request.id);

    let models: Vec<work_set::ActiveModel> = (0..request.count)
        .map(|_| {
            let mut clone = copy_model.clone().into_active_model();
            clone.id = NotSet;
            clone
        })
        .collect();

    Json(CRUDWorkSet::create_many(&state.db, models).await.unwrap())
}

pub async fn exercise_count_delete(
    State(state): State<AppState>,
    Json(request): Json<ExerciseCountDeleteRequest>,
) -> Json<u64> {
    Json(
        CRUDWorkSet::delete_many_by_ids(&state.db, request.work_set_ids)
            .await
            .unwrap(),
    )
}
