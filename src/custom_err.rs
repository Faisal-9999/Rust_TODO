use std::fmt::{self, Display, Formatter};
use std::error::Error;


#[derive(Debug)]
pub enum CustomError {
    WriteLineError,
    DatabaseLoadError,
    DatabaseSaveError,
}

impl Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        
        let message = match self {
            CustomError::WriteLineError => "Error While Writing Line To File",
            CustomError::DatabaseLoadError => "Error While Loading Database",
            CustomError::DatabaseSaveError => "Error While Saving Database",
        };

        write!(f, "{message}")
    }
}