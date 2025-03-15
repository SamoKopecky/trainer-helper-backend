use entity::person;
use sea_orm::entity::prelude::*;
use sea_orm::DatabaseConnection;

use super::ResultCRUD;

pub struct CRUDPerson;

impl CRUDPerson {
    pub async fn insert_person(
        db_conn: &DatabaseConnection,
        person: person::ActiveModel,
    ) -> ResultCRUD<person::Model> {
        Ok(person.insert(db_conn).await?)
    }

    pub async fn select_users(db_conn: &DatabaseConnection) -> ResultCRUD<Vec<person::Model>> {
        person::Entity::find().all(db_conn).await
    }

    pub async fn select_user_by_id(
        db_conn: &DatabaseConnection,
        id: i32,
    ) -> ResultCRUD<person::Model> {
        Ok(person::Entity::find()
            .filter(person::Column::Id.eq(id))
            .one(db_conn)
            .await?
            .unwrap())
    }
}
