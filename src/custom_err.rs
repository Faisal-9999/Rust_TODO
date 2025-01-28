use std::fmt::{self, Display};
use std::error::Error;


#[derive(Debug)]
#[derive(Clone)]
pub enum CustomError {
    WriteLineError,
    DatabaseLoadError,
    DatabaseSaveError,
    StringToDateConversionError,
    StringTou32ConversionError,
    InvalidIndexError,
}

impl Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        
        let message = match self {
            CustomError::WriteLineError => "Error While Writing Line To File",
            CustomError::DatabaseLoadError => "Error While Loading Database",
            CustomError::DatabaseSaveError => "Error While Saving Database",
            CustomError::StringToDateConversionError => "Error While Converting String To Date",
            CustomError::StringTou32ConversionError => "Error While Converting String to u32",
            CustomError::InvalidIndexError => "Invalid Index Entered Line Doesn't Exist",
        };

        write!(f, "{message}")
    }
}