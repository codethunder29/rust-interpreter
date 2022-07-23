use crate::scanner::token::*;

#[derive(Debug)]
pub struct RuntimeError {
    pub msg: String,
    pub token: Option<Token>
}

impl RuntimeError {
    pub fn get_message(&self) -> String {
        self.msg.clone()
    }
}