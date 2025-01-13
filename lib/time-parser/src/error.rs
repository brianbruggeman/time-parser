type Value = String;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid duration")]
    InvalidDuration(Value),

    #[error("invalid time")]
    InvalidTime(Value),

    #[error("invalid unit: {0}")]
    InvalidUnit(Value),

    #[error("{0}")]
    Error(String),
}

impl Error {
    #[expect(clippy::self_named_constructors)]
    pub fn error(value: impl ToString) -> Self {
        Self::Error(value.to_string())
    }

    pub fn invalid_duration(value: impl ToString) -> Self {
        Self::InvalidDuration(value.to_string())
    }

    pub fn invalid_time(value: impl ToString) -> Self {
        Self::InvalidTime(value.to_string())
    }

    pub fn invalid_unit(value: impl ToString) -> Self {
        Self::InvalidUnit(value.to_string())
    }
}
