use crate::token::Token;

pub enum Error {
    ScannerError {line: u32, msg: String},
    ParserError {token: Token, line: u32, msg: String}
}

impl Error {
    pub fn message(&self) -> String {
        match self {
            Error::ScannerError {line, msg} => {
                return format!("{} in line {}", msg, line);
            },
            Error::ParserError{token, line, msg} => {
                return format!("{} in line {}", msg, line);
            }
        }
    }
}