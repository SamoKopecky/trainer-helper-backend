use axum::http::StatusCode;
use chrono::NaiveDateTime;
use sea_orm::{
    ActiveValue::{self, NotSet},
    DbErr, Set,
};

use crate::crud::ResultCRUD;

pub fn active<T>(value: Option<T>) -> ActiveValue<T>
where
    T: Into<sea_orm::Value>,
{
    value.map_or(NotSet, Set)
}

pub fn handle_crud_result<T>(result: ResultCRUD<T>) -> StatusCode {
    match result {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => match e {
            DbErr::RecordNotUpdated => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        },
    }
}

pub fn datetime_to_human_date(date: NaiveDateTime) -> String {
    date.format("%d-%m").to_string()
}

pub fn datetime_to_human_time(date: NaiveDateTime) -> String {
    date.format("%H:%M").to_string()
}
