use sea_orm::{entity::prelude::*, sqlx::types::chrono::Utc, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "timeslot")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub trainer_id: i32,
    pub start: DateTime,
    pub duration: i32,
    #[serde(skip_serializing)]
    pub updated_at: DateTime,
    #[serde(skip_serializing)]
    pub created_at: DateTime,
    pub user_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn build(trainer_id: i32, start: DateTime, duration_minutes: i32) -> ActiveModel {
        let naive_now = Utc::now().naive_local();

        ActiveModel {
            trainer_id: Set(trainer_id),
            start: Set(start),
            duration: Set(duration_minutes),
            created_at: Set(naive_now),
            updated_at: Set(naive_now),
            user_id: Set(None),
            ..Default::default()
        }
    }
}
