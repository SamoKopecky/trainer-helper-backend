use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "timeslot")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub trainer_id: i32,
    pub start: DateTime,
    pub duration: i32,
    pub updated_at: DateTime,
    pub created_at: DateTime,
    pub user_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
