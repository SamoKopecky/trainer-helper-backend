use sea_orm::{
    ActiveValue::{self, NotSet},
    Set,
};

pub fn active<T>(value: Option<T>) -> ActiveValue<T>
where
    T: Into<sea_orm::Value>,
{
    value.map_or(NotSet, Set)
}
