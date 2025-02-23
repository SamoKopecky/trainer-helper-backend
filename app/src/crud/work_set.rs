use chrono::Utc;
use entity::prelude::*;
use entity::work_set;
use sea_orm::Order;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::{entity::prelude::*, Set};

use super::ResultCRUD;

pub struct CRUDWorkSet;

impl CRUDWorkSet {
    pub async fn create_many(
        db_conn: &DatabaseConnection,
        models: Vec<work_set::ActiveModel>,
    ) -> ResultCRUD<Vec<work_set::Model>> {
        WorkSet::insert_many(models)
            .exec_with_returning_many(db_conn)
            .await
    }

    pub async fn update_by_id(
        db_conn: &DatabaseConnection,
        id: i32,
        mut data: work_set::ActiveModel,
    ) -> ResultCRUD<work_set::Model> {
        data.id = Set(id);
        data.updated_at = Set(Utc::now().naive_local());
        data.update(db_conn).await
    }

    pub async fn get_work_set_count_by_exercise_id(
        db_conn: &DatabaseConnection,
        exercise_id: i32,
    ) -> ResultCRUD<i32> {
        let res = work_set::Entity::find()
            .filter(work_set::Column::ExerciseId.eq(exercise_id))
            .count(db_conn)
            .await?;
        Ok(res.try_into().expect("Can't convert u64 to i32"))
    }

    fn get_ordered_by_exercise_id_query(exercise_id: i32) -> Select<work_set::Entity> {
        work_set::Entity::find()
            .filter(work_set::Column::ExerciseId.eq(exercise_id))
            // TODO: sort properly
            .order_by(work_set::Column::Id, Order::Asc)
    }

    pub async fn get_last_by_exercise_id(
        db_conn: &DatabaseConnection,
        exercise_id: i32,
    ) -> ResultCRUD<Option<work_set::Model>> {
        Self::get_ordered_by_exercise_id_query(exercise_id)
            .one(db_conn)
            .await
    }

    pub async fn get_many_ordered_ids(
        db_conn: &DatabaseConnection,
        exercise_id: i32,
        limit: u64,
    ) -> ResultCRUD<Vec<i32>> {
        Self::get_ordered_by_exercise_id_query(exercise_id)
            .limit(limit)
            .select_only()
            .column(work_set::Column::Id)
            .into_tuple()
            .all(db_conn)
            .await
    }

    pub async fn delete_many_by_ids(
        db_conn: &DatabaseConnection,
        ids: Vec<i32>,
    ) -> ResultCRUD<u64> {
        Ok(WorkSet::delete_many()
            .filter(work_set::Column::Id.is_in(ids))
            .exec(db_conn)
            .await?
            .rows_affected)
    }
}
