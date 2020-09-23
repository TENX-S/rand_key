
use thiserror::Error;




#[derive(Error, Debug)]
pub enum GenError {

    #[error("The corresponding character is missing!")]
    MissChar,

    #[error("Delete non-exist value!")]
    DelNonExistValue,

    #[error("Non ASCII character")]
    InvalidChar,

    #[error("No `{0}` kind of field in `RandKey`")]
    InvalidKind(String),

    #[error("The `{0}` field is invalid")]
    InvalidField(String),

    #[error("Unit can not be zero")]
    UnitNeZero,

}