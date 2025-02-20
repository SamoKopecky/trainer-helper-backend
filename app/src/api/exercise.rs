use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use entity::exercise::{self, SetType};
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};

use crate::crud::exercise::CRUDExercise;

use super::{utils::active, AppState};

// TODO: Move models somewhere else, also rename many things to make sense
#[derive(Serialize, Deserialize, Debug)]
struct ApiExerciseWorkSet {
    pub work_set_id: i32,
    pub reps: i32,
    pub intensity: String,
    pub rpe: Option<i32>,
    // TODO: Add from_exercise function
}

// TODO: Add ordering for work sets vector or maybe just sort them by updated_at
#[derive(Serialize, Deserialize, Debug)]
struct ApiExercise {
    pub exercise_id: i32,
    pub group_id: i32,
    pub work_set_count: i32,
    pub set_type: SetType,
    pub note: Option<String>,
    pub work_sets: Vec<ApiExerciseWorkSet>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExercisePutRequest {
    pub id: i32,
    pub note: Option<String>,
}

pub async fn get_exercise(
    State(state): State<AppState>,
    Path(timeslot_id): Path<i32>,
) -> Json<Value> {
    let mut res: HashMap<i32, ApiExercise> = HashMap::new();
    let exercises = CRUDExercise::get_by_timeslot_id(&state.db, timeslot_id)
        .await
        .unwrap();
    exercises.iter().for_each(|e| {
        res.entry(e.exercise_id)
            .and_modify(|ae| {
                ae.work_sets.push(ApiExerciseWorkSet {
                    work_set_id: e.work_set_id,
                    reps: e.reps,
                    intensity: e.intensity.clone(),
                    rpe: e.rpe,
                });
                ae.work_set_count += 1
            })
            .or_insert(ApiExercise {
                exercise_id: e.exercise_id,
                group_id: e.group_id,
                note: e.note.clone(),
                set_type: e.set_type,
                work_set_count: 1,
                work_sets: vec![ApiExerciseWorkSet {
                    work_set_id: e.work_set_id,
                    reps: e.reps,
                    intensity: e.intensity.clone(),
                    rpe: e.rpe,
                }],
            });
    });

    let mut res_values: Vec<ApiExercise> = res.into_values().collect();
    res_values.sort_by_key(|k| (k.group_id, k.exercise_id));
    Json(to_value(res_values).unwrap())
}

pub async fn exercise_update(
    State(state): State<AppState>,
    Json(request): Json<ExercisePutRequest>,
) -> StatusCode {
    let update_model = exercise::ActiveModel {
        note: active(request.note.map(Some)),
        ..Default::default()
    };

    match CRUDExercise::update_by_id(&state.db, request.id, update_model).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => match e {
            DbErr::RecordNotUpdated => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        },
    }
}
