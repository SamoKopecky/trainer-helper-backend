use sea_orm::prelude::*;
use sea_orm::{sqlx::types::chrono::Utc, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "work_set")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub reps: i32,
    pub intensity: String,
    pub rpe: Option<i32>,
    #[serde(skip_serializing)]
    pub created_at: DateTime,
    #[serde(skip_serializing)]
    pub updated_at: DateTime,
    pub exercise_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn build(reps: i32, intensity: String, exercise_id: i32, rpe: Option<i32>) -> ActiveModel {
        let naive_now = Utc::now().naive_local();

        ActiveModel {
            reps: Set(reps),
            intensity: Set(intensity),
            rpe: Set(rpe),
            exercise_id: Set(exercise_id),
            created_at: Set(naive_now),
            updated_at: Set(naive_now),
            ..Default::default()
        }
    }
}
