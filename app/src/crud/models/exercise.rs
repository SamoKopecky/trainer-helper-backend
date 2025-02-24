use entity::exercise::SetType;
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
    pub work_set_count: Option<i32>,
}
