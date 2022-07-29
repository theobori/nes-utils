use std::{
    error::Error,
    fmt
};

#[derive(Debug)]
pub enum NesError {
    Arguments
}

impl fmt::Display for NesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NesError::Arguments => f.write_str("Arguments error")
        }
    }
}

impl Error for NesError {
    fn description(&self) -> &str {
        match *self {
            NesError::Arguments => "Invalid CLI arguments",
        }
    }
}
