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

    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Expr, Error> {
        self.tokens = tokens;
        
        self.expression()
    }

    // primary defines the start of the expression, for example this expr is valid "let a = 1000", this is not ") a = 1000"
    fn primary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![TokenType::Number, TokenType::String, TokenType::Bool]) {
            match self.previous().literal.as_ref().unwrap() {
                TokenLiteral::Int(val) => return Ok(Expr::Literal(Some(ExprLiteral::Int(*val)))),
                TokenLiteral::Float(val) => return Ok(Expr::Literal(Some(ExprLiteral::Float(*val)))),
                TokenLiteral::Str(val) => return Ok(Expr::Literal(Some(ExprLiteral::Str((*val).clone())))),
                TokenLiteral::Bool(val) => return Ok(Expr::Literal(Some(ExprLiteral::Bool(*val)))),
            }
        }

        if self.match_token(vec![TokenType::Null]) {
            return Ok(Expr::Literal(None))
        }

        if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression();

            match self.consume(TokenType::RightParen, "Expected ')' after experssion".to_string()) {
                Ok(_) => return Ok(Expr::Gropuing(Box::new(expr?))),
                Err(e) => return Err(e)
            }
        }

        let token = self.peek();

        Err(self.gen_error(format!("Invalid syntax '{}' in line {}", token.lexeme, token.line), token.clone()))
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = match self.previous().ttype {
                TokenType::Bang => Some(UnaryOp::Bang),
                TokenType::Minus => Some(UnaryOp::Minus),
                _ => None
            };

            let right = self.unary()?;
            return Ok(Expr::Unary(operator.unwrap(), Box::new(right)));
        }

        Ok(self.primary()?)
    }

    // multiplaction and division
    fn factor(&mut self) -> Result<Expr, Error> {
        let expr = self.unary()?;

        while self.match_token(vec![TokenType::Star, TokenType::Slash]) {
            let operator = match self.previous().ttype {
                TokenType::Star => Some(BinaryOp::Star),
                TokenType::Slash => Some(BinaryOp::Slash),
                _ => None
            };

            let right = self.unary()?;
            return Ok(Expr::BinaryOp(Box::new(expr), operator.unwrap(), Box::new(right)));
        }
        Ok(expr)
    }

    // Addition and subtraction
    fn term(&mut self) -> Result<Expr, Error> {
        let expr = self.factor()?;

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = match self.previous().ttype {
                TokenType::Minus => Some(BinaryOp::Minus),
                TokenType::Plus => Some(BinaryOp::Plus),
                _ => None
            };

            let right = self.factor()?;
            return Ok(Expr::BinaryOp(Box::new(expr), operator.unwrap(), Box::new(right)));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let expr = self.term()?;

        while self.match_token(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = match self.previous().ttype {
                TokenType::Greater => Some(BinaryOp::Greater),
                TokenType::GreaterEqual => Some(BinaryOp::GreaterEqual),
                TokenType::Less => Some(BinaryOp::Less),
                TokenType::LessEqual => Some(BinaryOp::LessEqual),
                _ => None
            };

            let right = self.term()?;
            return Ok(Expr::BinaryOp(Box::new(expr), operator.unwrap(), Box::new(right)));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let expr = self.comparison()?;

        // if the next token after epxr is == or != it means that expr is part of an equality else its another expr
        while self.match_token(vec![TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = match self.previous().ttype {
                TokenType::EqualEqual => Some(BinaryOp::EqualEqual),
                TokenType::BangEqual => Some(BinaryOp::BangEqual),
                _ => None
            };

            let right = self.comparison()?;
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
    fn gen_error(&self, msg: String, token: Token) -> Error {
        Error::ParserError {
            msg,
            token
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

    fn synchronize(&mut self) {
        if !self.at_end() {
            self.pos += 1;
        }

        while !self.at_end() {
            if self.previous().ttype == TokenType::Semicolon {
                return;
            }

            match self.peek().ttype {
                TokenType::Class => return,
                TokenType::Fn => return,
                TokenType::Let => return,
                TokenType::For => return,
                TokenType::If => return,
                TokenType::While => return,
                TokenType::Print => return,
                TokenType::Return => return,
                _ => {}
            }

            self.pos += 1;
        }
    }

    // runs until it finds token and return ok or it came to EOF and returns parser error with err_msg
    fn consume(&mut self, ttype: TokenType, err_msg: String) -> Result<(), Error> {
        while !self.at_end() && self.peek().ttype != ttype {
            self.pos += 1;
        }

        if self.at_end() {
            Err(self.gen_error(err_msg, self.peek().clone()))
        }
        else { 
            Ok(())
        }
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for ttype in types {
            if self.peek().ttype == ttype {
                if !self.at_end() {
                    self.pos += 1;
                }
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