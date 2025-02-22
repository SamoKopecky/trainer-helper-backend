use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TimeslotPutRequest {
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
}
