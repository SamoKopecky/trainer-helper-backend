use entity::{
    exercise::{self, SetType},
    work_set,
};
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
pub struct ExerciseResponse {
    pub exercise_id: i32,
    pub group_id: i32,
    pub work_set_count: i32,
    pub set_type: SetType,
    pub note: Option<String>,
    pub work_sets: Vec<ExerciseWorkSet>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FullExerciseResponse {
    pub name: String,
    pub user_id: Option<i32>,
    pub exercises: Vec<ExerciseResponse>,
}

#[derive(Deserialize, Debug)]
pub struct ExerciseCountPutRequest {
    pub id: i32,
    pub count: i32,
    pub work_set_template: ExerciseWorkSet,
}

#[derive(Deserialize, Debug)]
pub struct ExerciseCountDeleteRequest {
    pub work_set_ids: Vec<i32>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExercisePutRequest {
    pub id: i32,
    pub note: Option<String>,
    pub set_type: Option<SetType>,
    pub group_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct ExerciseDeleteRequest {
    pub timeslot_id: i32,
    pub exercise_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct ExercisePostRequest {
    pub timeslot_id: i32,
    pub group_id: i32,
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
    pub fn to_active_model(self, exercise_id: i32) -> work_set::ActiveModel {
        work_set::Entity::build(self.reps, self.intensity, exercise_id, self.rpe)
    }
}

impl ExerciseResponse {
    pub fn from_crud_model(model: &ExerciseWorkSetModel) -> Self {
        ExerciseResponse {
            exercise_id: model.exercise_id,
            group_id: model.group_id,
            note: model.note.clone(),
            set_type: model.set_type,
            work_set_count: 1,
            work_sets: vec![ExerciseWorkSet::from_crud_model(model)],
        }
    }
    pub fn from_crud_models(work_set: &work_set::Model, exercise: &exercise::Model) -> Self {
        ExerciseResponse {
            set_type: exercise.set_type,
            group_id: exercise.group_id,
            note: exercise.note.clone(),
            exercise_id: exercise.id,
            work_set_count: 1,
            work_sets: vec![ExerciseWorkSet {
                rpe: work_set.rpe,
                reps: work_set.reps,
                intensity: work_set.intensity.clone(),
                work_set_id: work_set.id,
            }],
        }
    }
}
