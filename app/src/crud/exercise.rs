use chrono::Utc;
use entity::{exercise, work_set};
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, Iterable, JoinType, QuerySelect, Set};

use super::models::exercise::ExerciseWorkSetModel;
use super::ResultCRUD;

pub struct CRUDExercise;

impl CRUDExercise {
    pub async fn create(
        db_conn: &DatabaseConnection,
        model: exercise::ActiveModel,
    ) -> ResultCRUD<exercise::Model> {
        model.insert(db_conn).await
    }

    pub async fn get_by_timeslot_id(
        db_conn: &DatabaseConnection,
        timeslot_id: i32,
    ) -> ResultCRUD<Vec<ExerciseWorkSetModel>> {
        let query = exercise::Entity::find()
            .filter(exercise::Column::TimeslotId.eq(timeslot_id))
            .join(JoinType::Join, exercise::Relation::WorkSet.def())
            .select_only()
            .columns(exercise::Column::iter().filter(|col| match col {
                exercise::Column::Id => false,
                _ => true,
            }))
            .columns(work_set::Column::iter().filter(|col| match col {
                work_set::Column::Id => false,
                _ => true,
            }))
            .column_as(work_set::Column::Id, "work_set_id")
            .column_as(exercise::Column::Id, "exercise_id");

        query.into_model().all(db_conn).await
    }

    pub async fn update_by_id(
        db_conn: &DatabaseConnection,
        id: i32,
        mut data: exercise::ActiveModel,
    ) -> ResultCRUD<exercise::Model> {
        data.id = Set(id);
        data.updated_at = Set(Utc::now().naive_local());
        data.update(db_conn).await
    }

    pub async fn delete_by_group_and_timeslot_id(
        db_conn: &DatabaseConnection,
        timeslot_id: i32,
        group_id: i32,
    ) -> ResultCRUD<()> {
        exercise::Entity::delete_many()
            .filter(
                exercise::Column::TimeslotId
                    .eq(timeslot_id)
                    .and(exercise::Column::GroupId.eq(group_id)),
            )
            .exec(db_conn)
            .await?;
        Ok(())
    }
}
