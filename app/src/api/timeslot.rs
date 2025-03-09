use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use entity::timeslot;

use crate::{api::utils::active, crud::timeslot::CRUDTimeslot};

use super::{
    schemas::timeslot::{
        TimeslotDeleteRequest, TimeslotGetQuery, TimeslotPostRequest, TimeslotPutRequest,
    },
    utils::{datetime_to_human_date, datetime_to_human_time},
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
    Json(request): Json<TimeslotPostRequest>,
) -> Json<timeslot::Model> {
    let timelost_name = format!(
        "from {} to {} on {}",
        datetime_to_human_time(request.start),
        datetime_to_human_time(request.end),
        datetime_to_human_date(request.start)
    );
    Json(
        CRUDTimeslot::insert_timeslot(
            &state.db,
            timeslot::Entity::build(
                request.trainer_id,
                request.start,
                request.end,
                timelost_name,
            ),
        )
        .await
        .unwrap(),
    )
}

pub async fn timeslot_delete(
    State(state): State<AppState>,
    Json(request): Json<TimeslotDeleteRequest>,
) -> Json<timeslot::Model> {
    Json(
        CRUDTimeslot::delete_timeslot(&state.db, request.timeslot_id)
            .await
            .unwrap(),
    )
}

pub async fn timeslot_put(
    State(state): State<AppState>,
    Json(request): Json<TimeslotPutRequest>,
) -> StatusCode {
    let update_model = timeslot::ActiveModel {
        name: active(request.name),
        user_id: active(request.user_id.map(Some)),
        ..Default::default()
    };

    CRUDTimeslot::update_by_id(&state.db, request.id, update_model)
        .await
        .unwrap();

    StatusCode::OK
}
