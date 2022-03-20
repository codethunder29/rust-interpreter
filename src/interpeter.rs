// use std::fs::{self, File};
// use std::io::{BufReader, BufRead, stdout, Write, Read};
// use self::token::{Token, TokenType, LiteralValue};

// mod token;


// fn scan_token(lex_buffer: &String) -> Token {
//     let mut token = Token::new(TokenType::Null, String::new(), None, 0);

//     // match chr {
        
//     //     _ if chr.is
//     //     _ => {}
//     // }

//     // token.lexeme = ;

//     token
// }

// fn scan_file(path: &str) -> Vec<Token> {
//     let source: Vec<char> = fs::read_to_string(path)
//                                 .expect("Could not find file")
//                                 .chars()
//                                 .collect();

//     let mut tokens = Vec::new();
//     let mut line = 1;
//     let mut in_brackets = false;
//     let mut lex_buffer = String::new();
//     let mut lex_vec: Vec<String> = Vec::new();
    
//     for i in 0..source.len() {
//         let ch = source[i];

//         match ch {
//             '(' => {},
//             ')' => {},
//             '{' => {},
//             '}' => {},
//             ',' => {},
//             '.' => {},
//             '-' => {},
//             '+' => {},
//             ';' => {},
//             '*' => {},
//             '!' => {},
//             '<' => {},
//             '>' => {},
//             '/' => {},
//             '=' => {},
//             '"' => {},
//             '0'..='9' => {},
//             '\n' => line += 1,
//             _ if ch.is_ascii_alphabetic() => {},
//             ' ' | '\r' | '\t' => {},
//             _ => panic!("ilia gay")
//         }
//     }

    

//     println!("{:?}", lex_vec);

//     // adding EOF token (its always the last token)
//     tokens.push(Token::new(TokenType::Eof, String::new(), None, line));

//     tokens
// }

// pub fn run_file(path: &str) {
//     let tokens = scan_file(path);
// }