use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    Json,
};
use entity::exercise::SetType;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};

use crate::crud::exercise::CRUDExercise;

use super::AppState;

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
    res_values.sort_by_key(|k| k.group_id);
    Json(to_value(res_values).unwrap())
}
