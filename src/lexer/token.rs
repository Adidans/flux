#[derive(Debug)]
#[allow(dead_code)]
pub enum TokenType {
    // Single-character tokens
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

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Types
    StringType,
    FloatType,
    IntType,
    BoolType,

    // Literals
    Identifier(String),
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),

    // Keywords
    And,
    Class,
    Else,
    False,
    Func,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: i64) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}
