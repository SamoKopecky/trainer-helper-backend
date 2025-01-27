use crate::models::timeslot::*;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::PgConnection;

use super::errors::CRUDResult;

pub struct TimeslotCRUD;

impl TimeslotCRUD {
    pub fn get_by_id(db_conn: &mut PgConnection, query_id: i32) -> CRUDResult<Option<Timeslot>> {
        use crate::schema::timeslots::dsl::*;

        Ok(timeslots
            .filter(id.eq(query_id))
            .first::<Timeslot>(db_conn)
            // Question mark return Result<E> if it happends
            .optional()?)
    }

    pub fn insert_many(
        db_conn: &mut PgConnection,
        insert_timeslots: Vec<NewTimeslot>,
    ) -> CRUDResult<usize> {
        use crate::schema::timeslots::dsl::*;

        Ok(insert_into(timeslots)
            .values(insert_timeslots)
            .execute(db_conn)?)
    }
}
