use sea_orm::DbErr;

pub mod exercise;
pub mod person;
pub mod timeslot;
pub mod work_set;

/// Contains all special models that might be required by CRUD
///
/// Most of the time models from `entity` crate are sufficient
/// If joins are used and such, special model needs to be created for the
/// result
///
/// Models are always postfixed with Model to label them as such
pub mod models;

pub type ResultCRUD<T> = Result<T, DbErr>;
