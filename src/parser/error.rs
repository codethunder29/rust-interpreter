use crate::scanner::token::*;

#[derive(Debug)]
pub struct ParserError {
    pub msg: String,
    pub token: Option<Token>
}

impl ParserError {
    pub fn get_message(&self) -> String {
        self.msg.clone()
    }

    pub fn print_msg(&self) {
        
    }
}