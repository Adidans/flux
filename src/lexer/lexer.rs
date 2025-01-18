use crate::lexer::token::{Token, TokenType};

pub struct Lexer {
    source: String,
    start: usize,
    current: usize,
    line: i64,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            if let Some(token) = self.scan_token() {
                tokens.push(token);
            }
        }

        tokens.push(Token::new(TokenType::Eof, self.line));
        tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance();

        match c {
            '(' => Some(self.make_token(TokenType::LeftParen)),
            ')' => Some(self.make_token(TokenType::RightParen)),
            '{' => Some(self.make_token(TokenType::LeftBrace)),
            '}' => Some(self.make_token(TokenType::RightBrace)),
            ',' => Some(self.make_token(TokenType::Comma)),
            '.' => Some(self.make_token(TokenType::Dot)),
            '-' => Some(self.make_token(TokenType::Minus)),
            '+' => Some(self.make_token(TokenType::Plus)),
            ';' => Some(self.make_token(TokenType::Semicolon)),
            '*' => Some(self.make_token(TokenType::Star)),
            '!' => {
                if self.match_next('=') {
                    Some(self.make_token(TokenType::BangEqual))
                } else {
                    Some(self.make_token(TokenType::Bang))
                }
            }
            '=' => {
                if self.match_next('=') {
                    Some(self.make_token(TokenType::EqualEqual))
                } else {
                    Some(self.make_token(TokenType::Equal))
                }
            }
            '<' => {
                if self.match_next('=') {
                    Some(self.make_token(TokenType::LessEqual))
                } else {
                    Some(self.make_token(TokenType::Less))
                }
            }
            '>' => {
                if self.match_next('=') {
                    Some(self.make_token(TokenType::GreaterEqual))
                } else {
                    Some(self.make_token(TokenType::Greater))
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    None
                } else {
                    Some(self.make_token(TokenType::Slash))
                }
            }
            // Handle whitespace
            ' ' | '\r' | '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }
            // Handle numbers
            '0'..='9' => Some(self.number()),
            // Handle identifiers or keywords
            'a'..='z' | 'A'..='Z' | '_' => Some(self.identifier()),
            // Unrecognized character
            _ => {
                eprintln!("Unexpected character: '{}' on line {}", c, self.line);
                None
            }
        }
    }

    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        char
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.line)
    }

    fn current_lexeme(&self) -> String {
        self.source[self.start..self.current].to_string()
    }

    fn number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance(); // Consume the '.'

            while self.peek().is_digit(10) {
                self.advance();
            }

            let lexeme = self.current_lexeme();
            return Token::new(TokenType::Float(lexeme.parse().unwrap()), self.line);
        }

        let lexeme = self.current_lexeme();
        Token::new(TokenType::Integer(lexeme.parse().unwrap()), self.line)
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let lexeme = self.current_lexeme();
        let token_type = self.keyword_or_identifier(&lexeme);
        Token::new(token_type, self.line)
    }

    fn keyword_or_identifier(&self, lexeme: &str) -> TokenType {
        match lexeme {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "null" => TokenType::Null,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            "int" => TokenType::IntType,
            "float" => TokenType::FloatType,
            "string" => TokenType::StringType,
            "bool" => TokenType::BoolType,
            _ => TokenType::Identifier(lexeme.to_string()),
        }
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }
}
