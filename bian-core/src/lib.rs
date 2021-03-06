pub mod enums;
pub mod error;

pub type BianResult<T> = Result<T, error::APIError>;
