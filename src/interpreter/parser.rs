use std::fmt::Binary;
use std::io::{stdout, Write};
use super::error::Error;
use super::expr::*;
use super::token::{Token, TokenType, TokenLiteral};

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
        
        match self.expression() {
            Ok(val) => {},
            Err(e) => println!("{:?}", e),
        }
        // self.expression().unwrap();
        // while !self.at_end() {

        // }
    }

    // primary defines the start of the expression, for example this expr is valid "let a = 1000", this is not ") a = 1000"
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
                println!("{}", self.pos);
                let expr = self.expression();

                match self.consume(TokenType::RightParen, "Expected ')' after experssion".to_string()) {
                    Ok(_) => return Ok(Expr::Gropuing(Box::new(expr.unwrap()))),
                    Err(e) => return Err(e)
                }
            },
            _ => {}
        }

        Err(self.gen_error(String::from("Error")))
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = match self.previous().ttype {
                TokenType::Bang => Some(UnaryOp::Bang),
                TokenType::Minus => Some(UnaryOp::Minus),
                _ => None
            };

            let right = self.unary().unwrap();
            return Ok(Expr::Unary(operator.unwrap(), Box::new(right)));
        }

        Ok(self.primary().unwrap())
    }

    // multiplaction and division
    fn factor(&mut self) -> Result<Expr, Error> {
        let expr = self.unary().unwrap();

        while self.match_token(vec![TokenType::Star, TokenType::Slash]) {
            let operator = match self.previous().ttype {
                TokenType::Star => Some(BinaryOp::Star),
                TokenType::Slash => Some(BinaryOp::Slash),
                _ => None
            };

            let right = self.unary().unwrap();
            return Ok(Expr::BinaryOp(Box::new(expr), operator.unwrap(), Box::new(right)));
        }
        Ok(expr)
    }

    // Addition and subtraction
    fn term(&mut self) -> Result<Expr, Error> {
        let expr = self.factor().unwrap();

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = match self.previous().ttype {
                TokenType::Minus => Some(BinaryOp::Minus),
                TokenType::Plus => Some(BinaryOp::Plus),
                _ => None
            };

            let right = self.factor().unwrap();
            return Ok(Expr::BinaryOp(Box::new(expr), operator.unwrap(), Box::new(right)));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let expr = self.term().unwrap();

        while self.match_token(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = match self.previous().ttype {
                TokenType::Greater => Some(BinaryOp::Greater),
                TokenType::GreaterEqual => Some(BinaryOp::GreaterEqual),
                TokenType::Less => Some(BinaryOp::Less),
                TokenType::LessEqual => Some(BinaryOp::LessEqual),
                _ => None
            };

            let right = self.term().unwrap();
            return Ok(Expr::BinaryOp(Box::new(expr), operator.unwrap(), Box::new(right)));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let expr = self.comparison().unwrap();

        // if the next token after epxr is == or != it means that expr is part of an equality else its another expr
        while self.match_token(vec![TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = match self.previous().ttype {
                TokenType::EqualEqual => Some(BinaryOp::EqualEqual),
                TokenType::BangEqual => Some(BinaryOp::BangEqual),
                _ => None
            };

            let right = self.comparison().unwrap();
            return Ok(Expr::BinaryOp(Box::new(expr), operator.unwrap(), Box::new(right)));
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }
}

// helper functions
impl Parser {
    fn gen_error(&self, msg: String) -> Error {
        Error::ParserError {
            msg
        }
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

    // runs until it finds token and return ok or it came to EOF and returns parser error with err_msg
    fn consume(&mut self, token: TokenType, err_msg: String) -> Result<(), Error> {
        while self.peek().ttype != token && !self.at_end() {
            self.pos += 1;
        }

        if self.at_end() {
            Ok(())
        }
        else {
            Err(self.gen_error(err_msg))
        }
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for ttype in types {
            if self.peek().ttype == ttype {
                self.pos += 1;
                return true;
            }
        }

        false
    }
}

pub fn print_tree_pretty(expr: Expr) {
    match expr {
        Expr::Literal(val) => {
            if val.is_some() {
                print!("{:?}", val.unwrap());
                stdout().flush().unwrap();
            }
        },
        Expr::Gropuing(val) => {
            print!("( ");
            print_tree_pretty(*val);
            print!(" )");
            stdout().flush().unwrap();
        },
        Expr::Unary(op,val) => {
            match op {
                UnaryOp::Minus => print!("-"),
                UnaryOp::Bang => print!("!")
            }

            print_tree_pretty(*val);
        },
        Expr::BinaryOp(val1, op, val2) => {
            print_tree_pretty(*val1);

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

            print_tree_pretty(*val2);
        }
    }
}

pub fn print_tree(expr: Expr) {
    match expr {
        Expr::Literal(val) => {
            print!("{:?}", val.unwrap());
            stdout().flush().unwrap();
        },
        Expr::Gropuing(val) => {
            print!("( ");
            print_tree(*val);
            print!(" )");
        },
        Expr::Unary(op,val) => {
            match op {
                UnaryOp::Minus => print!("-"),
                UnaryOp::Bang => print!("!")
            }

            print_tree(*val);
        },
        Expr::BinaryOp(val1, op, val2) => {
            match op {
                BinaryOp::EqualEqual => print!("== "),
                BinaryOp::BangEqual => print!("!= "),
                BinaryOp::Less => print!("< "),
                BinaryOp::LessEqual => print!("<= "),
                BinaryOp::Greater => print!("> "),
                BinaryOp::GreaterEqual => print!(">= "),
                BinaryOp::Plus => print!("+ "),
                BinaryOp::Minus => print!("- "),
                BinaryOp::Star => print!("* "),
                BinaryOp::Slash => print!("/ ")
            }

            print!("(");
            print_tree(*val1);
            print!(" ");
            print_tree(*val2);
            print!(")");
        }
    }
}