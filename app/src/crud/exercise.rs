use entity::{exercise, work_set};
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, JoinType, QuerySelect};

use super::ResultCRUD;

pub struct CRUDExercise;

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
    ) -> ResultCRUD<Vec<exercise::Model>> {
        exercise::Entity::find()
            .filter(exercise::Column::TimeslotId.eq(timeslot_id))
            .join(
                JoinType::Join,
                work_set::Entity::belongs_to(exercise::Entity)
                    .from(work_set::Column::ExerciseId)
                    .to(exercise::Column::Id)
                    .into(),
            )
            .all(db_conn)
            .await
    }
}
