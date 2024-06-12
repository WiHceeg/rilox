use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Literal {
    None,
    String(String),
    Number(f64),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,   // 实际的值
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}