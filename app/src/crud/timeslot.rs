use entity::timeslot;
use sea_orm::entity::prelude::*;

use super::ResultCRUD;

pub struct CRUDTimeslot;

impl CRUDTimeslot {
    pub async fn get_by_range_date(
        db_conn: &DatabaseConnection,
        start_date: DateTime,
        end_date: DateTime,
    ) -> ResultCRUD<Vec<timeslot::Model>> {
        timeslot::Entity::find()
            .filter(timeslot::Column::Start.between(start_date, end_date))
            .all(db_conn)
            .await
    }
}
