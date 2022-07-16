use super::token::*;
use super::error::Error;
use std::fs;

pub struct Scanner {
    source: Vec<char>,
    pos: u32,
    line: u32,
    tokens: Vec<Token>,
}

// main logic functions
impl Scanner {
    pub fn new(source_path: &String) -> Scanner {
        Scanner {
            source: fs::read_to_string(source_path).unwrap().chars().collect(),
            pos: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn from_str(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            tokens: Vec::new()
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        while !self.at_end() {
            match self.scan_token() {
                Ok(()) => {},
                Err(e) => {
                    return Err(e);
                }
            }
        }

        self.add_token(TokenType::Eof, String::from("eof"), None);
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let ch = self.source[self.pos as usize];

        match ch {
            '(' => {
                self.add_token(TokenType::LeftParen, String::from("("), None);
                self.pos += 1;
            }
            ')' => {
                self.add_token(TokenType::RightParen, String::from(")"), None);
                self.pos += 1;
            }
            '{' => {
                self.add_token(TokenType::LeftBrace, String::from("{"), None);
                self.pos += 1;
            }
            '}' => {
                self.add_token(TokenType::RightBrace, String::from("}"), None);
                self.pos += 1;
            }
            ',' => {
                self.add_token(TokenType::Comma, String::from(","), None);
                self.pos += 1;
            }
            '.' => {
                self.add_token(TokenType::Dot, String::from("."), None);
                self.pos += 1;
            }
            '-' => {
                self.add_token(TokenType::Minus, String::from("-"), None);
                self.pos += 1;
            }
            '+' => {
                self.add_token(TokenType::Plus, String::from("+"), None);
                self.pos += 1;
            }
            ';' => {
                self.add_token(TokenType::Semicolon, String::from(";"), None);
                self.pos += 1;
            }
            '*' => {
                self.add_token(TokenType::Star, String::from("*"), None);
                self.pos += 1;
            }
            '!' => {
                if self.check_next('=') {
                    self.add_token(TokenType::BangEqual, String::from("!="), None);
                    self.pos += 2;
                }
                else {
                    self.add_token(TokenType::Bang, String::from("!"), None);
                    self.pos += 1;
                }
            }
            '=' => {
                if self.check_next('=') {
                    self.add_token(TokenType::EqualEqual, String::from("=="), None);
                    self.pos += 2;
                }
                else {
                    self.add_token(TokenType::Equal, String::from("="), None);
                    self.pos += 1;
                }
            }
            '<' => {
                if self.check_next('=') {
                    self.add_token(TokenType::LessEqual, String::from("<="), None);
                    self.pos += 2;
                }
                else {
                    self.add_token(TokenType::Less, String::from("<"), None);
                    self.pos += 1;
                }
            }
            '>' => {
                if self.check_next('=') {
                    self.add_token(TokenType::GreaterEqual, String::from(">="), None);
                    self.pos += 2;
                }
                else {
                    self.add_token(TokenType::Greater, String::from(">"), None);
                    self.pos += 1;
                }
            }
            '/' => {
                // checking for comment
                if self.check_next('/') {
                    // looping to skip the rest of the entire line
                    while !self.at_end() {
                        if self.get(self.pos) == '\n' {
                            break;
                        }

                        self.pos += 1;
                    }
                }
                // checking for multi line comment
                else if self.check_next('*') {
                    let mut comments: Vec<u32> = Vec::new();

                    while !self.at_end() {
                        if self.get(self.pos) == '/' && self.check_next('*') {
                            comments.push(self.line);
                            self.pos += 2;
                        }
                        else if self.get(self.pos) == '*' && self.check_next('/') {
                            comments.pop();
                            self.pos += 2;
                            
                            if comments.len() == 0 {
                                break;
                            }
                        }
                        else {
                            if self.get(self.pos) == '\n' {
                                self.line += 1;
                            }

                            self.pos += 1;
                        }
                    }

                    if comments.len() > 0 {
                        return Err(self.gen_error(format!("Comment in line {} is not closed", comments.pop().unwrap())));
                    }
                }
                // regular slash
                else {
                    self.add_token(TokenType::Slash, String::from("/"), None);
                    self.pos += 1;
                }
            }
            '"' => {
                let string_start = self.line;
                let mut buffer = String::new();
                let mut closed = false;

                // will break on loop without goind forward
                self.pos += 1;

                while !self.at_end() {
                    let ch = self.get(self.pos);

                    if ch == '"' {
                        closed = true;
                        self.pos += 1; // doing this to skip over the closing quotation marks
                        break;
                    }
                    else if ch == '\n' {
                        self.line += 1;
                    }

                    buffer.push(ch);
                    self.pos += 1;
                }

                if closed == false {
                    return Err(self.gen_error(format!("String in line {} is not closed", string_start)));
                }
                else {
                    let mut lexeme = String::from('"');
                    lexeme.push_str(&buffer);
                    lexeme.push('"');
                    self.add_token(TokenType::String, lexeme, Some(TokenLiteral::Str(buffer)));
                }
            }
            '\n' => {
                self.line += 1;
                self.pos += 1;
            }
            '0'..='9' => {
                let mut buffer = String::new();
                let mut ch: char;

                while !self.at_end() {
                    ch = self.get(self.pos);

                    if !(ch.is_ascii_digit() || ch == '.') {
                        // checking for case like 100. or 100.abc
                        if buffer.chars().nth(buffer.len() - 1) == Some('.') {
                            return Err(self.gen_error(format!("Invalid syntax in line {}", self.line)))
                        }

                        break;
                    }

                    buffer.push(ch);
                    self.pos += 1;
                }

                if buffer.contains('.') {
                    let num: Result<f64, _> = buffer.parse();

                    match num {
                        Ok(num) => self.add_token(TokenType::Number, buffer, Some(TokenLiteral::Float(num))),
                        Err(_) => return Err(self.gen_error(format!("Invalid syntax '{}' in line {}", buffer, self.line)))
                    }  
                }
                else {
                    let num: i64 = buffer.parse().unwrap();
                    self.add_token(TokenType::Number, buffer, Some(TokenLiteral::Int(num)))
                }
            }
            _ if ch.is_alphabetic() || ch == '_' => {
                let mut buffer = String::new();
                let mut ch: char;

                while !self.at_end() {
                    ch = self.get(self.pos);

                    if !(ch.is_alphabetic() || ch == '_') {
                        break;
                    }

                    buffer.push(ch);
                    self.pos += 1;
                }

                self.check_keyword(&buffer);
            }
            ' ' | '\r' | '\t' => self.pos += 1,
            _ => {
                return Err(self.gen_error(format!("Unexpected character '{}' in line {}", &ch, self.line)));
            },
        }

        Ok(())
    }

    fn check_keyword(&mut self, word: &str) {
        match word {
            "if" => self.add_token(TokenType::If, String::from("if"), None),
            "else" => self.add_token(TokenType::Else, String::from("else"), None),
            "class" => self.add_token(TokenType::Class, String::from("class"), None),
            "fn" => self.add_token(TokenType::Fn, String::from("fn"), None),
            "for" => self.add_token(TokenType::For, String::from("for"), None),
            "while" => self.add_token(TokenType::While, String::from("while"), None),
            "null" => self.add_token(TokenType::Null, String::from("null"), None),
            "print" => self.add_token(TokenType::Print, String::from("print"), None),
            "return" => self.add_token(TokenType::Return, String::from("return"), None),
            "super" => self.add_token(TokenType::Super, String::from("super"), None),
            "this" => self.add_token(TokenType::This, String::from("this"), None),
            "let" => self.add_token(TokenType::Let, String::from("let"), None),
            "and" => self.add_token(TokenType::And, String::from("and"), None),
            "or" => self.add_token(TokenType::Or, String::from("or"), None),
            "true" => self.add_token(
                TokenType::Bool,
                String::from("true"),
                Some(TokenLiteral::Bool(true)),
            ),
            "false" => self.add_token(
                TokenType::Bool,
                String::from("false"),
                Some(TokenLiteral::Bool(false)),
            ),
            _ => self.add_token(TokenType::Identifier, String::from(word), None),
        }
    }
}

// helper functions
impl Scanner {
    fn add_token(&mut self, token_type: TokenType, lexeme: String, literal: Option<TokenLiteral>) {
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

    fn gen_error(&self, msg: String) -> Error {
        Error::ScannerError {
            msg
        }
    }
}