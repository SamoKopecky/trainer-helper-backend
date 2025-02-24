use axum::http::StatusCode;
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
