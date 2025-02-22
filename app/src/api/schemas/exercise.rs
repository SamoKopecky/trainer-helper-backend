use entity::exercise::SetType;
use serde::{Deserialize, Serialize};

use crate::crud::models::exercise::ExerciseWorkSetModel;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExerciseWorkSet {
    pub work_set_id: i32,
    pub reps: i32,
    pub intensity: String,
    pub rpe: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExerciseGetResponse {
    pub exercise_id: i32,
    pub group_id: i32,
    pub work_set_count: i32,
    pub set_type: SetType,
    pub note: Option<String>,
    pub work_sets: Vec<ExerciseWorkSet>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExercisePutRequest {
    pub id: i32,
    pub note: Option<String>,
}

impl ExerciseWorkSet {
    pub fn from_crud_model(model: &ExerciseWorkSetModel) -> Self {
        ExerciseWorkSet {
            work_set_id: model.work_set_id,
            reps: model.reps,
            intensity: model.intensity.clone(),
            rpe: model.rpe,
        }
    }
}

impl ExerciseGetResponse {
    pub fn from_crud_model(model: &ExerciseWorkSetModel) -> Self {
        ExerciseGetResponse {
            exercise_id: model.exercise_id,
            group_id: model.group_id,
            note: model.note.clone(),
            set_type: model.set_type,
            work_set_count: 1,
            work_sets: vec![ExerciseWorkSet::from_crud_model(model)],
        }
    }
}
