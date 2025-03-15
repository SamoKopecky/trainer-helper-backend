use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use entity::{
    exercise::{self, SetType},
    timeslot, work_set,
};
use sea_orm::{ActiveValue::NotSet, IntoActiveModel, Set};

use crate::crud::{exercise::CRUDExercise, timeslot::CRUDTimeslot, work_set::CRUDWorkSet};

use super::{
    schemas::{
        exercise::{
            ExerciseCountDeleteRequest, ExerciseCountPutRequest, ExerciseDeleteRequest,
            ExerciseDuplicatePostRequest, ExercisePostRequest, ExercisePutRequest,
            ExerciseResponse, ExerciseWorkSet, FullExerciseResponse,
        },
        timeslot::ApiTimeslot,
    },
    utils::{active, handle_crud_result},
    AppState,
};

pub async fn exercis_get(
    State(state): State<AppState>,
    Path(timeslot_id): Path<i32>,
) -> Json<FullExerciseResponse> {
    let mut res: HashMap<i32, ExerciseResponse> = HashMap::new();
    let exercises = CRUDExercise::get_by_timeslot_id_exercise_work_sets(&state.db, timeslot_id)
        .await
        .unwrap();
    exercises.iter().for_each(|exercise| {
        res.entry(exercise.exercise_id)
            .and_modify(|response| {
                response
                    .work_sets
                    .push(ExerciseWorkSet::from_big_crud_model(exercise));
                response.work_set_count += 1
            })
            .or_insert(ExerciseResponse::from_big_crud_model(exercise));
    });

    let mut res_exercises: Vec<ExerciseResponse> = res.into_values().collect();
    res_exercises.iter_mut().for_each(|exercise| {
        exercise
            .work_sets
            .sort_by_key(|work_set| work_set.work_set_id)
    });
    res_exercises.sort_by_key(|exercise| (exercise.group_id, exercise.exercise_id));

    let res_timeslot = CRUDTimeslot::get_by_id(&state.db, timeslot_id)
        .await
        .unwrap();
    Json(FullExerciseResponse {
        timeslot: ApiTimeslot::from_timeslot(&state.db, res_timeslot).await,
        exercises: res_exercises,
    })
}

pub async fn exercise_post(
    State(state): State<AppState>,
    Json(request): Json<ExercisePostRequest>,
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
    Json(ExerciseResponse::from_tuple_crud_models(
        new_work_set.first().unwrap(),
        &new_exercise,
    ))
}

pub async fn exercise_delete(
    State(state): State<AppState>,
    Json(request): Json<ExerciseDeleteRequest>,
) -> StatusCode {
    CRUDExercise::delete_by_exercise_and_timeslot_id(
        &state.db,
        request.timeslot_id,
        request.exercise_id,
    )
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
        group_id: active(request.group_id),
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

pub async fn exercise_duplicate(
    State(state): State<AppState>,
    Json(request): Json<ExerciseDuplicatePostRequest>,
) -> Json<FullExerciseResponse> {
    // TODO: Also update name and send timeslot

    let old_timeslot = CRUDTimeslot::get_by_id(&state.db, request.copy_timeslot_id)
        .await
        .unwrap();
    let updated_timeslot = CRUDTimeslot::update_by_id(
        &state.db,
        request.timeslot_id,
        timeslot::ActiveModel {
            name: Set(old_timeslot.name),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    CRUDExercise::delete_by_timeslot_id(&state.db, request.timeslot_id)
        .await
        .unwrap();

    let mut work_sets_map: HashMap<i32, Vec<work_set::Model>> = HashMap::new();

    let existing_exercises_join =
        CRUDExercise::get_by_timeslot_id(&state.db, request.copy_timeslot_id)
            .await
            .unwrap();
    let mut inserted_exercises: Vec<exercise::Model> = Vec::new();
    let mut new_work_sets: Vec<work_set::ActiveModel> = Vec::new();

    for exercise_join in existing_exercises_join {
        let mut new_exercise_model = exercise::Entity::to_new(exercise_join.0);
        new_exercise_model.timeslot_id = Set(request.timeslot_id);
        let new_exercise = CRUDExercise::create(&state.db, new_exercise_model)
            .await
            .unwrap();

        let mut news: Vec<work_set::ActiveModel> = exercise_join
            .1
            .into_iter()
            .map(|ws| {
                let mut new_ws = work_set::Entity::to_new(ws);
                new_ws.exercise_id = Set(new_exercise.id);
                new_ws
            })
            .collect();
        new_work_sets.append(&mut news);
        inserted_exercises.push(new_exercise);
    }

    let _ = CRUDWorkSet::create_many(&state.db, new_work_sets)
        .await
        .unwrap()
        .iter()
        .for_each(|ws| {
            work_sets_map
                .entry(ws.exercise_id)
                .and_modify(|ws_vec| ws_vec.push(ws.clone()))
                .or_insert(vec![ws.clone()]);
        });

    let res_exercises = inserted_exercises
        .into_iter()
        .map(|e| {
            let ws = work_sets_map.get(&e.id).unwrap();
            ExerciseResponse::from_crud_models(e, ws.to_vec())
        })
        .collect();

    Json(FullExerciseResponse {
        timeslot: ApiTimeslot::from_timeslot(&state.db, updated_timeslot).await,
        exercises: res_exercises,
    })
}
