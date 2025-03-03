use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TimeslotGetQuery {
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
}
