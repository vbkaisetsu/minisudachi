use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MinisudachiError {
    CsvError(csv::Error),
    ParseIntError(std::num::ParseIntError),
}

impl fmt::Display for MinisudachiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CsvError(e) => e.fmt(f),
            Self::ParseIntError(e) => e.fmt(f),
        }
    }
}

impl Error for MinisudachiError {}

impl From<csv::Error> for MinisudachiError {
    fn from(error: csv::Error) -> Self {
        Self::CsvError(error)
    }
}

impl From<std::num::ParseIntError> for MinisudachiError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::ParseIntError(error)
    }
}

pub type Result<T, E = MinisudachiError> = std::result::Result<T, E>;
