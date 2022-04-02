use std::io::{stdout, Write};
use crate::error::Error;
use crate::expr::*;
use crate::token::{Token, TokenType, TokenLiteral};

pub struct Parser {
    pos: u32,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            pos: 0,
            tokens: Vec::new()
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) {
        // let mut statments = Vec::new();
        self.tokens = tokens;
        

        while !self.at_end() {

        }
    }

    // primary defines the start of the expression, for example this expr is valid "var a = 1000", this is not ") a = 1000"
    fn primary(&mut self) -> Result<Expr, Error> {
        match self.peek().ttype {
            TokenType::Number | TokenType::String | TokenType::Bool => {
                // number can be integer or float
                match self.peek().literal.as_ref().unwrap() {
                    TokenLiteral::Int(val) => return Ok(Expr::Literal(Some(ExprLiteral::Int(*val)))),
                    TokenLiteral::Float(val) => return Ok(Expr::Literal(Some(ExprLiteral::Float(*val)))),
                    TokenLiteral::Str(val) => return Ok(Expr::Literal(Some(ExprLiteral::Str((*val).clone())))),
                    TokenLiteral::Bool(val) => return Ok(Expr::Literal(Some(ExprLiteral::Bool(*val)))),
                    _ => {}
                }
            },
            TokenType::Null => return Ok(Expr::Literal(None)),
            TokenType::LeftParen => {

            },
            _ => {}
        }

        Err(self.gen_error(String::from("Error")))
    }

    fn unary(&mut self) {

    }

    fn comparison(&mut self) {

    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let left = self.comparison();

        while self.match_token(vec![TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = match self.peek().ttype {
                TokenType::EqualEqual => Some(BinaryOp::EqualEqual),
                TokenType::BangEqual => Some(BinaryOp::BangEqual),
                _ => None
            };

            self.pos += 1;
            let right = self.comparison();
            return Ok(Expr::BinaryOp(Box::new(left), operator.unwrap(), Box::new(right)));
        }

        Err(self.gen_error(String::from(":/")))
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    fn gen_error(&self, msg: String) -> Error {
        Error::ParserError {
            msg
        }
    }

    fn match_token(&self, types: Vec<TokenType>) -> bool {
        for ttype in types {
            if self.peek().ttype == ttype {
                return true;
            }
        }

        false
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos as usize]
    }

    fn previous(&self) -> &Token {
        &self.tokens[(self.pos - 1) as usize]
    }

    fn next(&self) -> &Token {
        &self.tokens[(self.pos - 1) as usize]
    }

    fn at_end(&self) -> bool {
        self.pos as usize >= self.tokens.len()
    }
}

pub fn print_tree(expr: Expr) {
    match expr {
        Expr::Literal(val) => {
            if val.is_some() {
                print!("{:?}", val.unwrap());
                stdout().flush().unwrap();
            }
        },
        Expr::Gropuing(val) => {
            print!("( ");
            print_tree(*val);
            print!(" )");
            stdout().flush().unwrap();
        },
        Expr::Unary(op,val) => {
            match op {
                UnaryOp::Minus => print!("-"),
                UnaryOp::Bang => print!("!")
            }

            print_tree(*val);
        },
        Expr::BinaryOp(val1, op, val2) => {
            print_tree(*val1);

            match op {
                BinaryOp::EqualEqual => print!(" == "),
                BinaryOp::BangEqual => print!(" != "),
                BinaryOp::Less => print!(" < "),
                BinaryOp::LessEqual => print!(" <= "),
                BinaryOp::Greater => print!(" > "),
                BinaryOp::GreaterEqual => print!(" >= "),
                BinaryOp::Plus => print!(" + "),
                BinaryOp::Minus => print!(" - "),
                BinaryOp::Star => print!(" * "),
                BinaryOp::Slash => print!(" / ")
            }

            print_tree(*val2);
        }
    }
}