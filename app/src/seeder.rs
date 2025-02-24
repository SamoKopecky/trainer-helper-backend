use chrono::TimeDelta;
use chrono::Utc;
use entity::exercise;
use entity::exercise::SetType;
use entity::prelude::*;
use entity::timeslot;
use entity::work_set;
use sea_orm::prelude::*;

use crate::crud::exercise::CRUDExercise;
use crate::db::Db;

fn generate_sample_week() -> Vec<timeslot::ActiveModel> {
    let time = Utc::now().naive_local();
    const TRAINER_ID: i32 = 1;
    const DURATION: i32 = 60;

    (0..7)
        .map(|i| Timeslot::build(TRAINER_ID, time + TimeDelta::days(i), DURATION))
        .collect()
}

fn generate_exercises(timeslot_id: i32) -> Vec<exercise::ActiveModel> {
    let types = vec![SetType::Rdl, SetType::Squat];

    types
        .iter()
        .enumerate()
        .map(|(i, set_type)| Exercise::build(timeslot_id, i as i32, set_type.to_owned(), None))
        .collect()
}

fn generate_work_sets_in_timeslots(
    exercises: &mut Vec<exercise::Model>,
) -> Vec<work_set::ActiveModel> {
    let mut result: Vec<work_set::ActiveModel> = vec![];
    let squats_data = Vec::from([(4, "105Kg"), (3, "105Kg"), (6, "95Kg"), (5, "95Kg")]);
    let rdl_data = Vec::from([(7, "100Kg"), (7, "100Kg")]);

    let mut squats_sets = data_to_models(squats_data, exercises.pop().unwrap().id);
    let mut rdl_sets = data_to_models(rdl_data, exercises.pop().unwrap().id);
    result.append(&mut squats_sets);
    result.append(&mut rdl_sets);
    result
}

fn data_to_models(data: Vec<(i32, &str)>, exercise_id: i32) -> Vec<work_set::ActiveModel> {
    data.iter()
        .enumerate()
        .map(|(_, rdl)| WorkSet::build(rdl.0, rdl.1.to_string(), exercise_id, Some(7)))
        .collect()
}

pub async fn insert_seeds() {
    let db = Db::build().await.unwrap();
    let timeslots = generate_sample_week();
    let res = Timeslot::insert_many(timeslots)
        .exec_with_returning_many(&db.pool)
        .await
        .unwrap();

    println!("Inserted {} timeslots", res.len());

    let exercises = generate_exercises(res.last().unwrap().id);
    let mut res = Vec::new();
    for e in exercises {
        res.push(CRUDExercise::create(&db.pool, e).await.unwrap());
    }
    println!("Inserted {} exercises", res.len());

    let sets = generate_work_sets_in_timeslots(&mut res);
    let res = WorkSet::insert_many(sets)
        .exec_with_returning_many(&db.pool)
        .await
        .unwrap();
    println!("Inserted {} sets", res.len());
}
