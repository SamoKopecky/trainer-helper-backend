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
        ExerciseCountPutRequest, ExerciseCountPutResponse, ExerciseGetResponse, ExercisePutRequest,
        ExerciseWorkSet,
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
) -> Json<ExerciseCountPutResponse> {
    let old_count = CRUDWorkSet::get_work_set_count_by_exercise_id(&state.db, request.id)
        .await
        .unwrap();

    // TODO: Replace all unwraps with returning status code if its bad
    match request.work_set_count.cmp(&old_count) {
        std::cmp::Ordering::Greater => {
            let last_copy = CRUDWorkSet::get_last_by_exercise_id(&state.db, request.id)
                .await
                .unwrap()
                .expect("Exercise {} id doesn't have any work sets");
            let create_count = request.work_set_count - old_count;
            let models: Vec<work_set::ActiveModel> = (0..create_count)
                .map(|_| {
                    let mut clone = last_copy.clone().into_active_model();
                    clone.id = NotSet;
                    clone
                })
                .collect();
            let created_models = CRUDWorkSet::create_many(&state.db, models).await.unwrap();
            Json(ExerciseCountPutResponse {
                new_work_sets: created_models,
            })
        }
        std::cmp::Ordering::Less => {
            let delete_count = old_count - request.work_set_count;
            let ids_to_delete = CRUDWorkSet::get_many_ordered_ids(
                &state.db,
                request.id,
                delete_count.try_into().unwrap(),
            )
            .await
            .unwrap();
            let _ = CRUDWorkSet::delete_many_by_ids(&state.db, ids_to_delete).await;
            Json(ExerciseCountPutResponse {
                new_work_sets: Vec::new(),
            })
        }
        std::cmp::Ordering::Equal => Json(ExerciseCountPutResponse {
            new_work_sets: Vec::new(),
        }),
    }
}
