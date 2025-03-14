use sea_orm::{entity::prelude::*, sqlx::types::chrono::Utc, Set};
use serde::{Deserialize, Serialize};

use super::{
    exercise::{self},
    person,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "timeslot")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub trainer_id: i32,
    pub name: String,
    pub start: DateTime,
    pub end: DateTime,
    #[serde(skip_serializing)]
    pub updated_at: DateTime,
    #[serde(skip_serializing)]
    pub created_at: DateTime,
    pub user_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Exercise,
    Person,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Exercise => Entity::has_many(exercise::Entity).into(),
            Self::Person => Entity::has_one(person::Entity).into(),
        }
    }
}

impl Related<super::exercise::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Exercise.def()
    }
}

impl Related<super::person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn build(trainer_id: i32, start: DateTime, end: DateTime, name: String) -> ActiveModel {
        let naive_now = Utc::now().naive_local();

        ActiveModel {
            trainer_id: Set(trainer_id),
            name: Set(name),
            start: Set(start),
            end: Set(end),
            created_at: Set(naive_now),
            updated_at: Set(naive_now),
            user_id: Set(None),
            ..Default::default()
        }
    }
}
