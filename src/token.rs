#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // characters tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,
    Bool,

    // key words
    If,
    Else,
    Class,
    Fn,
    For,
    While,
    Null,
    Print,
    Return,
    Super,
    This,
    Var,
    And,
    Or,

    Eof,
}

#[derive(Debug, Clone)]
pub enum TokenLiteral {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType, // cant use type because its a keyword (ttype = token_type)
    pub lexeme: String,
    pub literal: Option<TokenLiteral>,
    pub line: u32,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<TokenLiteral>, line: u32) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line
        }
    }
}
