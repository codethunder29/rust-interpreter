use std::env;
use std::io::{stdin, stdout, Write};
use std::process;
use interpreter::Scanner;
use interpreter::Parser;
use interpreter::expr::*;
use interpreter::print_tree;

use crate::interpreter::parser::print_tree_pretty;

mod interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tree = Expr::BinaryOp(
        Box::new(Expr::Gropuing(
            Box::new(Expr::BinaryOp(
                Box::new(Expr::Literal(Some(ExprLiteral::Int(10)))),
                BinaryOp::Plus,
                Box::new( Expr::Literal(Some(ExprLiteral::Float(18.1)))),
            ))
        )),
        BinaryOp::Star,
        Box::new(Expr::Unary(
            UnaryOp::Minus,
            Box::new(Expr::Literal(Some(ExprLiteral::Int(200))))
        ))
    );
    
    // let tree = Node::new(Expr::Literal(None));
    print_tree(tree.clone());
    println!("");
    print_tree_pretty(tree);

    if args.len() > 2 {
        println!("Usage:\ninterpeter <script_path>");
        process::exit(1);
    } 
    else if args.len() == 1 {
        run_prompt();
    } 
    else {
        let mut scanner = Scanner::new(&args[1]);
        let mut parser = Parser::new();

        // println!("{:?}", scanner.scan_tokens());
        parser.parse(scanner.scan_tokens().unwrap());
    }
}

fn run_prompt() {
    let mut input = String::new();
    
    

    loop {
        input.clear();
        print!(">>> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let mut scanner = Scanner::from_str(input.clone());
        let mut parser = Parser::new();

        let tokens = scanner.scan_tokens();

        match tokens {
            Ok(val) => parser.parse(val),
            Err(e) => println!("{:?}", e)
        }
        // parser.parse(scanner.scan_tokens());
        // print!("{}", input);
    }
}
