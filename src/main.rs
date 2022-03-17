use std::env;
use std::io::{Write, stdin, stdout};
use std::process;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage:\ninterpeter <script_path>");
        process::exit(1);
    }
    else if args.len() == 1 {
        prompt();
    }
    else {
        lexer::scan_file();
    }
}


fn prompt() {
    let mut input = String::new();

    loop {
        input.clear();
        print!(">>> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        print!("{}", input);
    }
}
