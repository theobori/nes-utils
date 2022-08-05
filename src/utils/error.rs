use std::fmt;

#[derive(Debug)]
pub enum NesError {
    FileNotFound,
    FileInvalid,
    WrongNesFormat,
    HeaderNotParsed,
    NotImplementedOpcode,
    MissingChr
}

impl fmt::Display for NesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NesError::FileNotFound => write!(f, "File not found"),
            NesError::FileInvalid => write!(f, "Invalid file"),
            NesError::WrongNesFormat => write!(f, "Invalid NES format"),
            NesError::HeaderNotParsed => write!(f, "The NES header has not been parsed"),
            NesError::NotImplementedOpcode => write!(f, "NNot implemented operation code"),
            NesError::MissingChr => write!(f, "his program doesn't have a CHR ROM"),
        }
    }
}
