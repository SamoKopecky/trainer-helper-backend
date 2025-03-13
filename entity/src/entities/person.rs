use sea_orm::prelude::*;
use sea_orm::{sqlx::types::chrono::Utc, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "person")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub created_at: DateTime,
    #[serde(skip_serializing)]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn build(name: String, email: String) -> ActiveModel {
        let naive_now = Utc::now().naive_local();

        ActiveModel {
            name: Set(name),
            email: Set(email),
            created_at: Set(naive_now),
            updated_at: Set(naive_now),
            ..Default::default()
        }
    }
}
