use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct WorkSetPutRequest {
    pub id: i32,
    pub reps: Option<i32>,
    pub intensity: Option<String>,
    pub rpe: Option<i32>,
}
