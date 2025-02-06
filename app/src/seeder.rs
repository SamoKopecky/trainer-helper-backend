use chrono::TimeDelta;
use chrono::Utc;
use entity::prelude::*;
use entity::timeslot;

pub async fn generate_sample_week() -> Vec<timeslot::ActiveModel> {
    let time = Utc::now().naive_local();
    const TRAINER_ID: i32 = 1;
    const DURATION: i32 = 60;

    (0..7)
        .map(|i| Timeslot::build(TRAINER_ID, time + TimeDelta::days(i), DURATION))
        .collect()
}
