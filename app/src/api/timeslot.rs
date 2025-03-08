use axum::{
    extract::{Query, State},
    Json,
};
use entity::timeslot;

use crate::crud::timeslot::CRUDTimeslot;

use super::{
    schemas::timeslot::{TimeslotDeleteRequest, TimeslotGetQuery, TimeslotPostRequest},
    AppState,
};

pub async fn timeslot_get(
    State(state): State<AppState>,
    Query(params): Query<TimeslotGetQuery>,
) -> Json<Vec<timeslot::Model>> {
    Json(
        CRUDTimeslot::get_by_range_date(&state.db, params.start_date, params.end_date)
            .await
            .unwrap(),
    )
}

pub async fn timeslot_post(
    State(state): State<AppState>,
    Json(params): Json<TimeslotPostRequest>,
) -> Json<timeslot::Model> {
    Json(
        CRUDTimeslot::insert_timeslot(
            &state.db,
            timeslot::Entity::build(params.trainer_id, params.start, params.end),
        )
        .await
        .unwrap(),
    )
}

pub async fn timeslot_delete(
    State(state): State<AppState>,
    Json(params): Json<TimeslotDeleteRequest>,
) -> Json<timeslot::Model> {
    Json(
        CRUDTimeslot::delete_timeslot(&state.db, params.timeslot_id)
            .await
            .unwrap(),
    )
}
