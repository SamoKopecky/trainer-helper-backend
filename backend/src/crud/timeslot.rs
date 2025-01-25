use crate::models::*;
use diesel::prelude::*;
use diesel::PgConnection;

pub struct TimeslotCRUD;

impl TimeslotCRUD {
    pub fn get_by_id(db_conn: &mut PgConnection, query_id: i32) -> Vec<Timeslot> {
        use crate::schema::timeslots::dsl::*;

        timeslots
            .filter(id.eq(query_id))
            .select(Timeslot::as_select())
            .load(db_conn)
            .expect("bad")
    }
}
