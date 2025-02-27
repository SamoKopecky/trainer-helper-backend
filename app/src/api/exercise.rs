use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use entity::{
    exercise::{self, SetType},
    work_set,
};
use sea_orm::{ActiveValue::NotSet, IntoActiveModel};

use crate::crud::{exercise::CRUDExercise, work_set::CRUDWorkSet};

use super::{
    schemas::exercise::{
        ExerciseCountDeleteRequest, ExerciseCountPutRequest, ExercisePostDeleteRequest,
        ExercisePutRequest, ExerciseResponse, ExerciseWorkSet,
    },
    utils::{active, handle_crud_result},
    AppState,
};

pub async fn exercis_get(
    State(state): State<AppState>,
    Path(timeslot_id): Path<i32>,
) -> Json<Vec<ExerciseResponse>> {
    let mut res: HashMap<i32, ExerciseResponse> = HashMap::new();
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
            .or_insert(ExerciseResponse::from_crud_model(exercise));
    });

    let mut res_values: Vec<ExerciseResponse> = res.into_values().collect();
    res_values.iter_mut().for_each(|exercise| {
        exercise
            .work_sets
            .sort_by_key(|work_set| work_set.work_set_id)
    });
    res_values.sort_by_key(|exercise| (exercise.group_id, exercise.exercise_id));
    Json(res_values)
}

pub async fn exercise_post(
    State(state): State<AppState>,
    Json(request): Json<ExercisePostDeleteRequest>,
) -> Json<ExerciseResponse> {
    let new_exercise = CRUDExercise::create(
        &state.db,
        exercise::Entity::build(
            request.timeslot_id,
            request.group_id,
            SetType::None,
            Some("".to_string()),
        ),
    )
    .await
    .unwrap();
    let new_work_set = CRUDWorkSet::create_many(
        &state.db,
        vec![work_set::Entity::build(
            0,
            "-".to_string(),
            new_exercise.id,
            None,
        )],
    )
    .await
    .unwrap();
    Json(ExerciseResponse::from_crud_models(
        new_work_set.first().unwrap(),
        &new_exercise,
    ))
}

pub async fn exercise_delete(
    State(state): State<AppState>,
    Json(request): Json<ExercisePostDeleteRequest>,
) -> StatusCode {
    CRUDExercise::delete_by_group_and_timeslot_id(&state.db, request.timeslot_id, request.group_id)
        .await
        .unwrap();

    // TODO: Handle status code right
    StatusCode::OK
}

pub async fn exercise_put(
    State(state): State<AppState>,
    Json(request): Json<ExercisePutRequest>,
) -> StatusCode {
    let update_model = exercise::ActiveModel {
        set_type: active(request.set_type),
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
