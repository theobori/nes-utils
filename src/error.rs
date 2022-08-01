use std::{
    error::Error,
    fmt
};

#[derive(Debug)]
pub enum NesError {
    Arguments,
    FileNotFound,
    FileInvalid,
    WrongNesFormat
}

impl fmt::Display for NesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NesError::Arguments => f.write_str("Arguments error"),
            NesError::FileNotFound => f.write_str("FileNotFound error"),
            NesError::FileInvalid => f.write_str("FileInvalid error"),
            NesError::WrongNesFormat => f.write_str("WrongNesFormat error"),
        }
    }
}

impl Error for NesError {
    fn description(&self) -> &str {
        match *self {
            NesError::Arguments => "Invalid CLI arguments",
            NesError::FileNotFound => "File not found",
            NesError::FileInvalid => "Invalid file",
            NesError::WrongNesFormat => "Invalid NES format",
        }
    }
}
