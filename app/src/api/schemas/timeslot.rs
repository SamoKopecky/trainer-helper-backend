use chrono::NaiveDateTime;
use entity::timeslot;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::crud::{models::timeslot::PersonTimeslot, person::CRUDPerson};

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

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TimeslotPutRequest {
    pub id: i32,
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ApiTimeslot {
    #[serde(flatten)]
    pub timeslot: timeslot::Model,
    pub person_name: Option<String>,
}

impl ApiTimeslot {
    pub fn from_crud(timeslot_person: PersonTimeslot) -> Self {
        ApiTimeslot {
            timeslot: timeslot_person.timeslot,
            person_name: timeslot_person.person_name,
        }
    }

    pub async fn from_timeslot(db: &DatabaseConnection, timeslot: timeslot::Model) -> Self {
        let user = timeslot.user_id;

        let mut api_res = ApiTimeslot {
            timeslot,
            person_name: None,
        };

        if let Some(user_id) = user {
            let person = CRUDPerson::select_user_by_id(db, user_id).await.unwrap();

            api_res.person_name = Some(person.name);
        }

        api_res
    }
}
