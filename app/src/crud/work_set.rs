use chrono::Utc;
use entity::work_set;
use sea_orm::{entity::prelude::*, QueryOrder, Set};

pub struct CRUDWorkSet;

impl CRUDWorkSet {
    pub async fn get_by_timeslot_id(
        db_conn: &DatabaseConnection,
    ) -> Result<Vec<work_set::Model>, DbErr> {
        work_set::Entity::find().all(db_conn).await
    }

    pub async fn update_one_by_id(
        db_conn: &DatabaseConnection,
        id: i32,
        mut data: work_set::ActiveModel,
    ) -> Result<work_set::Model, DbErr> {
        data.id = Set(id);
        data.updated_at = Set(Utc::now().naive_local());
        data.update(db_conn).await
    }
}
