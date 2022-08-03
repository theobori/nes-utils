use std::{
    error::Error,
    fmt
};

#[derive(Debug)]
pub enum NesError {
    CLIArguments,
    FileNotFound,
    FileInvalid,
    WrongNesFormat,
    HeaderNotParsed,
    NotImplementedOpcode,
}

impl fmt::Display for NesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NesError::CLIArguments => f.write_str("CLIArguments error"),
            NesError::FileNotFound => f.write_str("FileNotFound error"),
            NesError::FileInvalid => f.write_str("FileInvalid error"),
            NesError::WrongNesFormat => f.write_str("WrongNesFormat error"),
            NesError::HeaderNotParsed => f.write_str("HeaderNotParsed error"),
            NesError::NotImplementedOpcode => f.write_str("NotImplementedOpcode error"),
        }
    }
}

impl Error for NesError {
    fn description(&self) -> &str {
        match *self {
            NesError::CLIArguments => "Invalid CLI arguments",
            NesError::FileNotFound => "File not found",
            NesError::FileInvalid => "Invalid file",
            NesError::WrongNesFormat => "Invalid NES format",
            NesError::HeaderNotParsed => "The NES header has not been parsed",
            NesError::NotImplementedOpcode => "Not implemented operation code",
        }
    }
}
