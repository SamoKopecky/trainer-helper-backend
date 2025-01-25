use self::models::*;
use crud::timeslot::TimeslotCRUD;
use db::PostgreClient;
use diesel::prelude::*;
use settings::AppConfig;
use std::{
    thread,
    time::{Duration, SystemTime},
};

pub mod crud;
mod db;
pub mod models;
pub mod schema;
mod settings;

fn main() {
    let app_config = AppConfig::build().expect("Error building configuration");
    let mut pg = PostgreClient::build(&app_config).unwrap();

    insert_to_db(&mut pg);
    // select_timeslots(&mut pg);

    for timeslot in TimeslotCRUD::get_by_id(&mut pg.client, 1) {
        println!("{}", timeslot.id);
    }

    println!("Running ...");
    thread::sleep(Duration::MAX);
}

fn insert_to_db(pg: &mut PostgreClient) -> Timeslot {
    use crate::schema::timeslots;

    let new_timeslot = NewTimeslot::new(1, SystemTime::now(), 60);

    diesel::insert_into(timeslots::table)
        .values(&new_timeslot)
        .returning(Timeslot::as_returning())
        .get_result(&mut pg.client)
        .expect("error while inserting")
}
