use sea_orm::{entity::prelude::*, sqlx::types::chrono::Utc, Set};
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
#[sea_orm(table_name = "exercise")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub timeslot_id: i32,
    pub group_id: i32,
    pub set_type: SetType,
    pub note: String,
    #[serde(skip_serializing)]
    pub updated_at: DateTime,
    #[serde(skip_serializing)]
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn build(timeslot_id: i32, group_id: i32, set_type: SetType, note: String) -> ActiveModel {
        let naive_now = Utc::now().naive_local();

        ActiveModel {
            timeslot_id: Set(timeslot_id),
            group_id: Set(group_id),
            set_type: Set(set_type),
            note: Set(note),
            created_at: Set(naive_now),
            updated_at: Set(naive_now),
            ..Default::default()
        }
    }
}
