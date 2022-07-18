use super::Token;

#[derive(Debug)]
pub enum Error {
    ScannerError {msg: String, line: u32, pos: u32},
    ParserError {msg: String}
}

impl Error {
    pub fn message(&self) -> String {
        match self {
            Error::ScannerError {msg, line, pos} => {
                return msg.clone();
            },
            Error::ParserError{msg} => {
                return msg.clone();
            }
        }
    }
}