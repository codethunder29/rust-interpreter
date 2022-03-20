#[derive(Debug)]
pub enum TokenType {
    // characters tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    Bang, BangEqual, Equal, EqualEqual, Greater,
    GreaterEqual, Less, LessEqual,

    // literals
    Identifier, String, Number,

    // key words
    If, Else, Class, True, False, Fn, For, While, Null,
    Print, Return, Super, This, Var, And, Or,

    Eof
}

#[derive(Debug)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    Str(String)
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,  // cant use type because its a keyword (ttype = token_type)
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line: u32
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<LiteralValue>, line: u32) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line
        }
    }
}