use std::env;
use std::io::{stdin, stdout, Write};
use std::process;
use scanner::Scanner;
use parser::{Parser, print_tree};
use expr::*;

mod scanner;
mod parser;
mod token;
mod expr;
mod error;


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
    print_tree(tree);
    println!("");

    if args.len() > 2 {
        println!("Usage:\ninterpeter <script_path>");
        process::exit(1);
    } else if args.len() == 1 {
        run_prompt();
    } else {
        let mut scanner = Scanner::new(&args[1]);
        let mut parser = Parser::new();

        parser.parse(scanner.scan_tokens());
    }
}

fn run_prompt() {
    let mut input = String::new();

    loop {
        input.clear();
        print!(">>> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        print!("{}", input);
    }
}
