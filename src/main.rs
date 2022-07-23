use std::env;
use std::io::{stdin, stdout, Write};
use std::process;
use scanner::Scanner;
use parser::Parser;
use interpreter::Interpreter;

mod scanner;
mod parser;
mod interpreter;

const VERSION: &str = "0.1";

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = vec![String::from(""), "scripts/test.scr".to_string()];

    // temp name for the language
    println!("Welcome to LoxScript {}", VERSION);

    if args.len() > 2 {
        println!("Usage:\ninterpeter <script_path>");
        process::exit(1);
    } 
    else if args.len() == 1 {
        run_prompt();
    } 
    else {
        let mut scanner = Scanner::new();
        let mut parser = Parser::new();
        let mut interpreter = Interpreter::new();

        let tokens = scanner.scan_from_file(&args[1]);
        if tokens.is_err() {
            let error = tokens.unwrap_err();
            error.print_msg();
            return;
        }

        let tokens = tokens.unwrap();
        let statements = parser.parse(tokens);

        if statements.is_err() {
            println!("{:?}", statements.unwrap_err());
            return;
        }

        let statements = statements.unwrap();
        let err = interpreter.interpret(statements);

        if err.is_err() {
            println!("{:?}", err.is_err());
            return;
        }

        // println!("{:?}", value.unwrap());

    }
}

fn run_prompt() {
    let mut input = String::new();
    let mut scanner = Scanner::new();
    let mut parser = Parser::new();
    let mut interpreter = Interpreter::new();
    
    loop {
        input.clear();
        scanner.reset();
        parser.reset();

        print!(">>> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        let tokens = scanner.scan_from_string(input.clone());

        // if tokens.is_err() {
        //     let error = tokens.unwrap_err();
        //     error.print_msg();
        //     continue;
        // }

        // let tokens = tokens.unwrap();
        // let ast = parser.parse(tokens);

        // if ast.is_err() {
        //     println!("{:?}", ast.unwrap_err());
        //     continue;
        // }

        // let ast = ast.unwrap();
        // let value = interpreter.interpret(ast);

        // if value.is_err() {
        //     println!("{:?}", value.is_err());
        //     continue;
        // }

        // println!("{:?}", value.unwrap());
    }
}
