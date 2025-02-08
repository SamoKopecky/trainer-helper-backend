use std::collections::HashMap;

use chrono::TimeDelta;
use chrono::Utc;
use entity::prelude::*;
use entity::timeslot;
use entity::work_set;

pub async fn generate_sample_week() -> Vec<timeslot::ActiveModel> {
    let time = Utc::now().naive_local();
    const TRAINER_ID: i32 = 1;
    const DURATION: i32 = 60;

    (0..7)
        .map(|i| Timeslot::build(TRAINER_ID, time + TimeDelta::days(i), DURATION))
        .collect()
}

pub async fn generate_work_sets_in_timeslots(timeslot_id: i32) -> Vec<work_set::ActiveModel> {
    let mut result: Vec<work_set::ActiveModel> = vec![];
    let squats_data = Vec::from([(4, "105Kg"), (3, "105Kg"), (6, "95Kg"), (5, "95Kg")]);
    let rdl_data = Vec::from([(7, "100Kg"), (7, "100Kg")]);
    let squat_type = "squat";
    let rdl_type = "rdl";

    let mut squats_sets: Vec<work_set::ActiveModel> = squats_data
        .iter()
        .enumerate()
        .map(|(i, squat)| {
            WorkSet::build(
                timeslot_id,
                i as i32,
                squat_type.to_string(),
                squat.0,
                squat.1.to_string(),
                Some(7),
                None,
                None,
            )
        })
        .collect();
    let mut rld_sets: Vec<work_set::ActiveModel> = rdl_data
        .iter()
        .enumerate()
        .map(|(i, rdl)| {
            WorkSet::build(
                timeslot_id,
                (i + squats_sets.len()) as i32,
                rdl_type.to_string(),
                rdl.0,
                rdl.1.to_string(),
                Some(7),
                None,
                None,
            )
        })
        .collect();
    result.append(&mut squats_sets);
    result.append(&mut rld_sets);
    result
}
