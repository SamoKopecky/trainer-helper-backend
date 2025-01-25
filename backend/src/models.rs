use std::time::SystemTime;

use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::timeslots)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Timeslot {
    pub id: i32,
    pub trainer_id: i32,
    pub user_id: Option<i32>,
    pub start: SystemTime,
    pub duration: i32,
    pub updated_at: SystemTime,
    pub created_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::timeslots)]
pub struct NewTimeslot {
    pub trainer_id: i32,
    pub start: SystemTime,
    pub duration: i32,
    pub updated_at: SystemTime,
    pub created_at: SystemTime,
}

impl NewTimeslot {
    pub fn new(trainer_id: i32, start: SystemTime, duration: i32) -> NewTimeslot {
        NewTimeslot {
            trainer_id,
            start,
            duration,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}
