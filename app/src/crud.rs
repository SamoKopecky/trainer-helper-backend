use sea_orm::DbErr;

pub mod exercise;
pub mod timeslot;
pub mod work_set;

type ResultCRUD<T> = Result<T, DbErr>;
