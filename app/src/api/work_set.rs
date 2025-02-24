use axum::{extract::State, http::StatusCode, Json};
use entity::work_set;

use crate::crud::work_set::CRUDWorkSet;

use super::{
    schemas::work_set::WorkSetPutRequest,
    utils::{active, handle_crud_result},
    AppState,
};

pub async fn work_set_put(
    State(state): State<AppState>,
    Json(request): Json<WorkSetPutRequest>,
) -> StatusCode {
    let update_model = work_set::ActiveModel {
        reps: active(request.reps),
        intensity: active(request.intensity),
        rpe: active(request.rpe.map(Some)),
        ..Default::default()
    };

    handle_crud_result(CRUDWorkSet::update_by_id(&state.db, request.id, update_model).await)
}
