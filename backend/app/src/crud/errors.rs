use diesel::result::Error as DieselError;

#[derive(Debug)]
pub enum CRUDError {
    TodoCustomError(String),
    DieselError(DieselError),
}

impl From<DieselError> for CRUDError {
    fn from(value: DieselError) -> Self {
        CRUDError::DieselError(value)
    }
}

pub type CRUDResult<T> = Result<T, CRUDError>;
