use super::Token;

#[derive(Debug)]
pub enum Error {
    ScannerError {msg: String},
    ParserError {msg: String}
}

impl Error {
    pub fn message(&self) -> String {
        match self {
            Error::ScannerError {msg} => {
                return format!("{}", msg);
            },
            Error::ParserError{msg} => {
                return format!("{}", msg);
            }
        }
    }
}