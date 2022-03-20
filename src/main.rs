use std::env;
use std::io::{Write, stdin, stdout};
use std::process;

mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage:\ninterpeter <script_path>");
        process::exit(1);
    }
    else if args.len() == 1 {
        run_prompt();
    }
    else {
        // interpeter::run_file(&args[1]);
        let mut scanner = scanner::Scanner::new(&args[1]);
        scanner.scan_tokens();
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
