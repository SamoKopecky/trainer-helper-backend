use axum::{extract::State, Json};
use entity::person;

use crate::crud::person::CRUDPerson;

use super::AppState;

pub async fn get_person(State(state): State<AppState>) -> Json<Vec<person::Model>> {
    Json(CRUDPerson::select_users(&state.db).await.unwrap())
}
