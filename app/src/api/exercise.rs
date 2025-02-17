use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};

use crate::crud::exercise::{CRUDExercise, ExerciseWorkSets};

use super::AppState;

// TODO: Move models somewhere else, also rename many things to make sense
#[derive(Serialize, Deserialize)]
struct ApiExerciseWorkSet {
    pub work_set_id: i32,
    pub reps: i32,
    pub intensity: String,
    pub rpe: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct ApiExercise {
    pub exercise_id: i32,
    pub group_id: i32,
    pub work_set_count: i32,
    pub note: Option<String>,
    pub work_sets: Vec<ApiExerciseWorkSet>,
}

pub async fn get_exercise(
    State(state): State<AppState>,
    Path(timeslot_id): Path<i32>,
) -> Json<Value> {
    let mut res: HashMap<i32, ApiExercise> = HashMap::new();
    let mut exercises = CRUDExercise::get_by_timeslot_id(&state.db, timeslot_id)
        .await
        .unwrap();
    let counts = get_work_set_counts(&exercises);
    exercises.iter_mut().for_each(|e| {
        res.entry(e.exercise_id)
            .and_modify(|ae| {
                ae.work_sets.push(ApiExerciseWorkSet {
                    work_set_id: e.work_set_id,
                    reps: e.reps,
                    // TODO: Find out what are possible solutions here
                    intensity: e.intensity.clone(),
                    rpe: e.rpe,
                })
            })
            .or_insert(ApiExercise {
                exercise_id: e.exercise_id,
                group_id: e.group_id,
                note: e.note.clone(),
                work_set_count: *counts.get(&e.exercise_id).unwrap(),
                work_sets: vec![ApiExerciseWorkSet {
                    work_set_id: e.work_set_id,
                    reps: e.reps,
                    intensity: e.intensity.clone(),
                    rpe: e.rpe,
                }],
            });
    });

    Json(to_value(res.values().collect::<Vec<&ApiExercise>>()).unwrap())
}

fn get_work_set_counts(exercises: &Vec<ExerciseWorkSets>) -> HashMap<i32, i32> {
    let mut res = HashMap::new();
    exercises.iter().for_each(|ews| {
        res.entry(ews.exercise_id)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });
    res
}
