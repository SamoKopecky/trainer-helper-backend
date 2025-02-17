use sea_orm::{prelude::*, Unchanged};
use sea_orm::{sqlx::types::chrono::Utc, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, Clone, Copy)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum SetType {
    Squat,
    Rdl,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "work_set")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub timeslot_id: i32,
    pub timeslot_index: i32,
    pub set_type: SetType,
    pub reps: i32,
    pub intensity: String,
    pub rpe: Option<i32>,
    pub tempo: Option<String>,
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: DateTime,
    #[serde(skip_serializing)]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn build(
        timeslot_id: i32,
        timeslot_index: i32,
        set_type: SetType,
        reps: i32,
        intensity: String,
        rpe: Option<i32>,
        tempo: Option<String>,
        note: Option<String>,
    ) -> ActiveModel {
        let naive_now = Utc::now().naive_local();

        ActiveModel {
            timeslot_id: Set(timeslot_id),
            timeslot_index: Set(timeslot_index),
            set_type: Set(set_type),
            reps: Set(reps),
            intensity: Set(intensity),
            rpe: Set(rpe),
            tempo: Set(tempo),
            note: Set(note),
            created_at: Set(naive_now),
            updated_at: Set(naive_now),
            ..Default::default()
        }
    }
}
