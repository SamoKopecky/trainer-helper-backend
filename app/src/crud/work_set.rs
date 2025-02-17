use chrono::Utc;
use entity::prelude::*;
use entity::work_set;
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
}
