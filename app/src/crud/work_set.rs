use entity::work_set;
use sea_orm::{entity::prelude::*, QueryOrder};

pub struct CRUDWorkSet;

impl CRUDWorkSet {
    pub async fn get_by_timeslot_id(
        db_conn: &DatabaseConnection,
        timeslot_id: i32,
    ) -> Result<Vec<work_set::Model>, DbErr> {
        work_set::Entity::find()
            .filter(work_set::Column::TimeslotId.eq(timeslot_id))
            .order_by_asc(work_set::Column::TimeslotIndex)
            .all(db_conn)
            .await
    }
}
