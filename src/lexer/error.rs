#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub line: i64,
    pub column: usize,
    pub span: (usize, usize),
}

impl LexerError {
    pub fn new(message: &str, line: i64, column: usize, span: (usize, usize)) -> Self {
        LexerError {
            message: message.to_string(),
            line,
            column,
            span,
        }
    }
}
