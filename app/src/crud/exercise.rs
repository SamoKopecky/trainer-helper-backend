use chrono::Utc;
use entity::exercise::SetType;
use entity::{exercise, work_set};
use sea_orm::entity::prelude::*;
use sea_orm::{
    DatabaseConnection, DbErr, FromQueryResult, Iterable, JoinType, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};

use super::ResultCRUD;

pub struct CRUDExercise;

// TODO: Move to a better place
#[derive(FromQueryResult, Serialize, Deserialize, Debug)]
pub struct ExerciseWorkSets {
    pub exercise_id: i32,
    pub timeslot_id: i32,
    pub work_set_id: i32,
    pub group_id: i32,
    pub set_type: SetType,
    pub intensity: String,
    pub rpe: Option<i32>,
    pub reps: i32,
    pub note: Option<String>,
    pub work_set_count: Option<i32>,
}

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
    ) -> ResultCRUD<Vec<ExerciseWorkSets>> {
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
}
