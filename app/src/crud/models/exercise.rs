use entity::{
    exercise::{self, SetType},
    work_set,
};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct ExerciseWorkSetModel {
    pub exercise_id: i32,
    pub timeslot_id: i32,
    pub work_set_id: i32,
    pub group_id: i32,
    pub set_type: SetType,
    pub intensity: String,
    pub rpe: Option<i32>,
    pub reps: i32,
    pub note: Option<String>,
}

impl ExerciseWorkSetModel {
    pub fn to_work_set_model(model: &Self, exercise_id: &i32) -> work_set::ActiveModel {
        work_set::Entity::build(model.reps, model.intensity.clone(), *exercise_id, model.rpe)
    }

    pub fn to_exercise_model(model: &Self, timeslot_id: i32) -> exercise::ActiveModel {
        exercise::Entity::build(
            timeslot_id,
            model.group_id,
            model.set_type,
            model.note.clone(),
        )
    }

    pub fn from_models(exercise: exercise::Model, work_set: work_set::Model) -> Self {
        Self {
            exercise_id: exercise.id,
            set_type: exercise.set_type,
            group_id: exercise.group_id,
            timeslot_id: exercise.timeslot_id,
            note: exercise.note,
            rpe: work_set.rpe,
            intensity: work_set.intensity,
            work_set_id: work_set.id,
            reps: work_set.reps,
        }
    }
}
