use axum::{extract::State, http::StatusCode, Json};
use entity::work_set;
use sea_orm::{
    ActiveValue::{self, NotSet},
    DbErr, Set,
};
use serde::Deserialize;
use serde_json::{to_value, Value};

use crate::crud::work_set::CRUDWorkSet;

use super::AppState;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct WorkSetPostRequest {
    timeslot_id: i32,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct WorkSetPutRequest {
    pub id: i32,
    pub reps: Option<i32>,
    pub intensity: Option<String>,
    pub rpe: Option<i32>,
}

// pub async fn work_set_post(
//     State(state): State<AppState>,
//     Json(_request): Json<WorkSetPostRequest>,
// ) -> Json<Value> {
//     let work_sets = CRUDWorkSet::get_by_timeslot_id(&state.db).await.unwrap();
//     Json(to_value(work_sets).unwrap())
// }

fn active<T>(value: Option<T>) -> ActiveValue<T>
where
    T: Into<sea_orm::Value>,
{
    value.map_or(NotSet, Set)
}

pub async fn work_set_update(
    State(state): State<AppState>,
    Json(request): Json<WorkSetPutRequest>,
) -> StatusCode {
    let update_model = work_set::ActiveModel {
        reps: active(request.reps),
        intensity: active(request.intensity),
        rpe: active(request.rpe.map(Some)),
        ..Default::default()
    };

    match CRUDWorkSet::update_by_id(&state.db, request.id, update_model).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => match e {
            DbErr::RecordNotUpdated => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        },
    }
}
