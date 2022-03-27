use std::io::{stdout, Write};
use crate::expr::*;
use crate::token::Token;

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
        self.tokens = tokens;

        // while !is_
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