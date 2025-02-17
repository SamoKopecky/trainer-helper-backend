use chrono::TimeDelta;
use chrono::Utc;
use entity::prelude::*;
use entity::timeslot;
use entity::work_set;

pub fn generate_sample_week() -> Vec<timeslot::ActiveModel> {
    let time = Utc::now().naive_local();
    const TRAINER_ID: i32 = 1;
    const DURATION: i32 = 60;

    (0..7)
        .map(|i| Timeslot::build(TRAINER_ID, time + TimeDelta::days(i), DURATION))
        .collect()
}

pub fn generate_work_sets_in_timeslots() -> Vec<work_set::ActiveModel> {
    let mut result: Vec<work_set::ActiveModel> = vec![];
    let squats_data = Vec::from([(4, "105Kg"), (3, "105Kg"), (6, "95Kg"), (5, "95Kg")]);
    let rdl_data = Vec::from([(7, "100Kg"), (7, "100Kg")]);

    let mut squats_sets = data_to_models(squats_data);
    let mut rdl_sets = data_to_models(rdl_data);
    result.append(&mut squats_sets);
    result.append(&mut rdl_sets);
    result
}

fn data_to_models(data: Vec<(i32, &str)>) -> Vec<work_set::ActiveModel> {
    data.iter()
        .enumerate()
        .map(|(_, rdl)| WorkSet::build(rdl.0, rdl.1.to_string(), Some(7)))
        .collect()
}
