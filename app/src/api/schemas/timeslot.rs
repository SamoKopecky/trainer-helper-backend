use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TimeslotGetQuery {
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TimeslotPostRequest {
    pub trainer_id: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct TimeslotDeleteRequest {
    pub timeslot_id: i32,
}
