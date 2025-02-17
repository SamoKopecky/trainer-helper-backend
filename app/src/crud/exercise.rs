use entity::{exercise, work_set};
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseBackend, DatabaseConnection, DbErr, QueryTrait};

use super::ResultCRUD;

pub struct CRUDExercise;

type All = Vec<(exercise::Model, Option<work_set::Model>)>;

impl CRUDExercise {
    pub async fn create(
        db_conn: &DatabaseConnection,
        model: exercise::ActiveModel,
    ) -> Result<exercise::Model, DbErr> {
        model.insert(db_conn).await
    }

    pub async fn get_by_timeslot_id(
        db_conn: &DatabaseConnection,
        timeslot_id: i32,
    ) -> ResultCRUD<All> {
        // TODO: Possible imporvment use data-loader in sea orm
        let test = exercise::Entity::find()
            .filter(exercise::Column::TimeslotId.eq(timeslot_id))
            .find_also_related(work_set::Entity)
            .build(DatabaseBackend::Postgres)
            .to_string();
        println!("{}", test);
        exercise::Entity::find()
            .filter(exercise::Column::TimeslotId.eq(timeslot_id))
            .find_also_related(work_set::Entity)
            .all(db_conn)
            .await
    }
}
