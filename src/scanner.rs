use std::{fs, io::LineWriter};
use self::token::{Token, TokenType, LiteralValue};

mod token;


pub struct Scanner {
    source: Vec<char>,
    pos: u32,
    line: u32,
    tokens: Vec<Token>
}

impl Scanner {
    pub fn new(source_path: &String) -> Scanner {
        Scanner {
            source: fs::read_to_string(source_path)
                        .unwrap()
                        .chars()
                        .collect(),
            pos: 0,
            line: 1,
            tokens: Vec::new()
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.at_end() {
            self.scan_token();
        }

        println!("{:?}", self.tokens);
    }

    fn scan_token(&mut self) {
        let ch = self.source[self.pos as usize];

        match ch {
            '(' => {
                self.add_token(TokenType::LeftParen, String::from("("), None);
                self.pos += 1;    
            },
            ')' => {
                self.add_token(TokenType::RightParen, String::from(")"), None);
                self.pos += 1;
            },
            '{' => {
                self.add_token(TokenType::LeftBrace, String::from("{"), None);
                self.pos += 1;
            },
            '}' => {
                self.add_token(TokenType::RightBrace, String::from("}"), None);
                self.pos += 1;
            },
            ',' => {
                self.add_token(TokenType::Comma, String::from(","), None);
                self.pos += 1;
            },
            '.' => {
                self.add_token(TokenType::Dot, String::from("."), None);
                self.pos += 1;
            },
            '-' => {
                self.add_token(TokenType::Minus, String::from("-"), None);
                self.pos += 1;
            },
            '+' => {
                self.add_token(TokenType::Plus, String::from("+"), None);
                self.pos += 1;
            },
            ';' => {
                self.add_token(TokenType::Semicolon, String::from(";"), None);
                self.pos += 1;
            },
            '*' => {
                self.add_token(TokenType::Star, String::from("*"), None);
                self.pos += 1;
            },
            '!' => {
                if self.check_next('=') {
                    self.add_token(TokenType::BangEqual, String::from("!="), None);
                    self.pos += 2;
                }
                else {
                    self.add_token(TokenType::Bang, String::from("!"), None);
                    self.pos += 1;
                }
            },
            '=' => {
                if self.check_next('=') {
                    self.add_token(TokenType::EqualEqual, String::from("=="), None);
                    self.pos += 2;
                }
                else {
                    self.add_token(TokenType::Equal, String::from("="), None);
                    self.pos += 1;
                }
            },
            '<' => {
                if self.check_next('=') {
                    self.add_token(TokenType::LessEqual, String::from("<="), None);
                    self.pos += 2;
                }
                else {
                    self.add_token(TokenType::Less, String::from("<"), None);
                    self.pos += 1;
                }
            },
            '>' => {
                if self.check_next('=') {
                    self.add_token(TokenType::GreaterEqual, String::from(">="), None);
                    self.pos += 2;
                }
                else {
                    self.add_token(TokenType::Greater, String::from(">"), None);
                    self.pos += 1;
                }
            },
            '/' => {
                if self.check_next('/') {
                    // looping to skip the rest of the entire line
                    while !self.at_end() {
                        if self.get(self.pos) == '\n' {
                            break;
                        }

                        self.pos += 1;
                    }
                }
                else {
                    self.add_token(TokenType::Slash, String::from("/"), None);
                    self.pos += 1;
                }
            },
            '"' => {
                let mut buffer = String::new();
                let mut closed = false;

                // will break on loop without goind forward
                self.pos += 1;

                while !self.at_end() {
                    let ch = self.get(self.pos);
                    
                    if ch == '"' {
                        closed = true;
                        self.pos += 1;  // doing this to skip over the closing quotation marks
                        break;
                    }

                    buffer.push(ch);
                    self.pos += 1;
                }

                if closed == false {
                    panic!();
                }
                else {
                    let mut lexeme = String::from('"');
                    lexeme.push_str(&buffer);
                    lexeme.push('"');
                    self.add_token(TokenType::String, lexeme, Some(LiteralValue::Str(buffer)));
                }
            },
            '\n' => {
                self.line += 1;
                self.pos += 1;
            },
            '0'..='9' => {
                let mut buffer = String::new();
                let mut ch: char;

                // NOTE: maybe add check for a case like 102.asada
                while !self.at_end() {
                    ch = self.get(self.pos);
                    
                    if !(ch.is_ascii_digit() || ch == '.') {
                        break;
                    }

                    buffer.push(ch);
                    self.pos += 1;
                }

                if buffer.contains('.') {
                    // NOTE: need to check for numbers like 455.12321.213
                    let num: f64 = buffer.parse().unwrap();
                    self.add_token(TokenType::Number, buffer, Some(LiteralValue::Float(num)))
                }
                else {
                    let num: i64 = buffer.parse().unwrap();
                    self.add_token(TokenType::Number, buffer, Some(LiteralValue::Int(num)))
                }
            },
            _ if ch.is_alphabetic() || ch == '_' => {
                self.pos += 1
            },
            ' ' | '\r' | '\t' => self.pos += 1,
            _ => panic!()
        }

    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String, literal: Option<LiteralValue>) {
        self.tokens.push(Token::new(token_type, lexeme, literal, self.line));
    }

    fn check_next(&self, ch: char) -> bool {
        if ((self.pos + 1) as usize) < self.source.len() {
            return self.get(self.pos + 1) == ch;
        }
        
        false
    }

    fn get(&self, idx: u32) -> char {
        self.source[idx as usize]
    }

    fn at_end(&self) -> bool {
        self.pos as usize >= self.source.len()
    }
}