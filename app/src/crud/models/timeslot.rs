use entity::timeslot;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonTimeslot {
    pub timeslot: timeslot::Model,
    pub person_name: Option<String>,
}
